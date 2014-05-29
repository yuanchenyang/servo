use std::cast;
use std::cell::RefCell;
use std::util;
use layout;
use layout::model::MaybeAuto::;
use layout::box_::Box;
use layout::flow::{Flow, BlockFlowClass,InlineFlowClass};
use layout::display_list_builder::{ExtraDisplayListData,ToGfxColor};

use servo_util::geometry::Au;
use geom::{Point2D, Rect, Size2D, SideOffsets2D};
use style::computed_values::{LengthOrPercentageOrAuto, LPA_Auto, border_style};
use gfx::display_list::{DisplayListCollection, DisplayList, BaseDisplayItem,
                        BorderDisplayItem, BorderDisplayItemClass};
use gfx::display_list::{SolidColorDisplayItem, SolidColorDisplayItemClass};
use layout::util::OpaqueNode;

// The root of the DOM tree, used by FTL
// pub struct RootFlow {
//     base: BaseFlow,
//
//     box_: Option<Box>,
//
//     ftl_attrs: layout::ftl_layout::RootFlowFtlAttrs,
//
//     screen_size: Rect<Au>
// }
//
// impl RootFlow {
//     pub fn from_layout_root(layout_root: Flow, ctx: &LayoutContext) -> RootFlow {
//         RootFlow {
//             base: flow::base(layout_root).clone(),
//             box_: layout_root.box_,
//             ftl_attrs: layout::ftl_layout::RootFlowFtlAttrs::new(),
//             screen_size: ctx.screen_size
//         }
//     }
// }

pub type DLCE = DisplayListCollection<OpaqueNode>;
pub type DLE = DisplayList<OpaqueNode>;

pub fn isEven( num: int ) -> bool {
    num % 2 == 0
}

pub fn base<'a,I>(node: &'a mut layout::ftl_layout::FtlNode) -> &'a mut I {
    unsafe {
        let (_, ptr) : (uint, uint) = cast::transmute(node);
        cast::transmute(ptr)
    }
}

pub fn specOrZero(length: LengthOrPercentageOrAuto, containing: Au) -> Au {
    //for b in flowbox.iter() {
    MaybeAuto::from_style(length, containing).specified_or_zero()
    //}
    //Au(0)
}

pub fn makeRect(x: Au, y: Au, width: Au, height: Au) -> Rect<Au> {
    Rect(Point2D(x, y),
         Size2D(width, height))
}

pub fn isAuto(length : LengthOrPercentageOrAuto) -> bool {
    match length {
        LPA_Auto => true,
        _ => false
    }

}

pub fn max (a : Au, b : Au) -> Au {
    if (a > b) {
        a
    } else {
        b
    }
}

pub fn newDisplayList() -> DLE {
    DisplayList::<OpaqueNode>::new()
}

pub fn newDisplayListCollection() -> DLCE {
    DisplayListCollection::<OpaqueNode>::new()
}

pub fn extendLists<E:ExtraDisplayListData>(collection: &mut DisplayListCollection<E>,
                                           list: &mut DisplayList<E>) -> int {
    collection.add_list(util::replace(list, DisplayList::<E>::new()));
    1
}

pub fn extendCollection<E:ExtraDisplayListData>(collection: &mut DisplayListCollection<E>,
                                                childCollection: &mut DisplayListCollection<E>) -> int {
    for list in childCollection.lists.mut_iter() {
        extendLists(collection, list);
    }
    1
}

pub fn addBorder<E:ExtraDisplayListData>(list: &mut DisplayList<E>, box_: &Box,
                                         x: Au, y: Au, width: Au, height: Au,
                                         t: Au, r: Au, b: Au, l: Au) -> int {

    let style = box_.style();
    let top_color = style.resolve_color(style.Border.get().border_top_color);
    let right_color = style.resolve_color(style.Border.get().border_right_color);
    let bottom_color = style.resolve_color(style.Border.get().border_bottom_color);
    let left_color = style.resolve_color(style.Border.get().border_left_color);
    let top_style = style.Border.get().border_top_style;
    let right_style = style.Border.get().border_right_style;
    let bottom_style = style.Border.get().border_bottom_style;
    let left_style = style.Border.get().border_left_style;

    let border = SideOffsets2D::new(t, r, b, l);
    let border_display_item = ~BorderDisplayItem {
        base: BaseDisplayItem {
            bounds: makeRect(x, y, width, height),
            extra: ExtraDisplayListData::new(box_),
        },
        border: border,
        color: SideOffsets2D::new(top_color.to_gfx_color(),
                                  right_color.to_gfx_color(),
                                  bottom_color.to_gfx_color(),
                                  left_color.to_gfx_color()),
        style: SideOffsets2D::new(top_style,
                                  right_style,
                                  bottom_style,
                                  left_style)
    };
    list.append_item(BorderDisplayItemClass(border_display_item));
    1
}

pub fn addBackground<E:ExtraDisplayListData>(list: &mut DisplayList<E>, box_: &Box,
                                             x: Au, y: Au, width: Au, height: Au) -> int {
    let style = box_.style();
    let background_color = style.resolve_color(style.Background.get().background_color);
    if !background_color.alpha.approx_eq(&0.0) {
        let solid_color_display_item = ~SolidColorDisplayItem {
            base: BaseDisplayItem {
                bounds: makeRect(x, y, width, height),
                extra: ExtraDisplayListData::new(box_),
            },
            color: background_color.to_gfx_color(),
        };
        list.append_item(SolidColorDisplayItemClass(solid_color_display_item));
    }
    1
}

pub fn rectHeight(rect: Rect<Au>) -> Au {
    rect.size.height
}

pub fn rectWidth(rect: Rect<Au>) -> Au {
    rect.size.width
}

pub fn inherit(visit: |&mut layout::ftl_layout::FtlNode|, node: &mut layout::ftl_layout::FtlNode) {
    visit(node);
    node.with_all_children(|child| {
        inherit(|node: &mut layout::ftl_layout::FtlNode| visit(node), child);
    });
}

pub fn synthesize(visit: |&mut layout::ftl_layout::FtlNode|,node: &mut layout::ftl_layout::FtlNode) {
    node.with_all_children(|child| {
        synthesize(|node: &mut layout::ftl_layout::FtlNode| visit(node), child);
    });
    visit(node);
}

pub fn log(logstr: &str){
    println!("{:s}",logstr)
}

pub fn as_ftl_node<'a>(flow: &'a mut Flow) -> &'a mut layout::ftl_layout::FtlNode {
    match flow.class() {
        BlockFlowClass => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        InlineFlowClass => flow.as_inline() as &'a mut layout::ftl_layout::FtlNode,
    }
}
