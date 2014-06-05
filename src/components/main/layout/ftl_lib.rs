use std::cast;
use std::mem;
use layout;
use layout::model::MaybeAuto::;
use layout::fragment::Fragment;
use layout::flow::{Flow, BlockFlowClass,InlineFlowClass,TableWrapperFlowClass,
                   TableFlowClass,TableColGroupFlowClass,TableRowGroupFlowClass,
                   TableRowFlowClass,TableCaptionFlowClass,TableCellFlowClass};
use layout::util::ToGfxColor;

use geom::approxeq::ApproxEq;

use servo_util::geometry::Au;
use geom::{Point2D, Rect, Size2D, SideOffsets2D};
use style::computed_values::{LengthOrPercentageOrAuto, LPA_Auto};
use gfx::display_list::{DisplayList, BaseDisplayItem,
                        BorderDisplayItem, BorderDisplayItemClass};
use gfx::display_list::{SolidColorDisplayItem, SolidColorDisplayItemClass,
                        BackgroundAndBordersStackingLevel};

// The root of the DOM tree, used by FTL
// pub struct RootFlow {
//     base: BaseFlow,
//
//     frag: Option<Box>,
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
//             frag: layout_root.frag,
//             ftl_attrs: layout::ftl_layout::RootFlowFtlAttrs::new(),
//             screen_size: ctx.screen_size
//         }
//     }
// }

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

pub fn newDisplayList() -> DisplayList {
    DisplayList::new()
}

pub fn mergeLists(parent: &mut DisplayList,
                  child:  &mut DisplayList) -> int {
    parent.push_all_move(mem::replace(child, DisplayList::new()));
    1
}

pub fn addBorder(list: &mut DisplayList, frag: &Fragment,
                 x: Au, y: Au, width: Au, height: Au,
                 t: Au, r: Au, b: Au, l: Au) -> int {

    let style = frag.style();

    let top_color = style.resolve_color(style.get_border().border_top_color);
    let right_color = style.resolve_color(style.get_border().border_right_color);
    let bottom_color = style.resolve_color(style.get_border().border_bottom_color);
    let left_color = style.resolve_color(style.get_border().border_left_color);

    let border = SideOffsets2D::new(t, r, b, l);
    let border_display_item = box BorderDisplayItem {
        base: BaseDisplayItem::new(makeRect(x, y, width, height),
                                   frag.node,
                                   BackgroundAndBordersStackingLevel),
        border: border,
        color: SideOffsets2D::new(top_color.to_gfx_color(),
                                  right_color.to_gfx_color(),
                                  bottom_color.to_gfx_color(),
                                  left_color.to_gfx_color()),
        style: SideOffsets2D::new(style.get_border().border_top_style,
                                      style.get_border().border_right_style,
                                      style.get_border().border_bottom_style,
                                      style.get_border().border_left_style)
    };
    list.push(BorderDisplayItemClass(border_display_item));
    1
}

pub fn addBackground(list: &mut DisplayList, frag: &Fragment,
                     x: Au, y: Au, width: Au, height: Au) -> int {
    let style = frag.style();
    let background_color = style.resolve_color(style.get_background().background_color);
    if !background_color.alpha.approx_eq(&0.0) {
        let solid_color_display_item = box SolidColorDisplayItem {
            base: BaseDisplayItem::new(makeRect(x, y, width, height),
                                       frag.node,
                                       BackgroundAndBordersStackingLevel),
            color: background_color.to_gfx_color(),
        };
        list.push(SolidColorDisplayItemClass(solid_color_display_item));
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
        BlockFlowClass         => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        InlineFlowClass        => flow.as_inline() as &'a mut layout::ftl_layout::FtlNode,
        TableColGroupFlowClass => flow.as_table_colgroup() as &'a mut layout::ftl_layout::FtlNode,
        TableWrapperFlowClass  => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        TableFlowClass         => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        TableRowGroupFlowClass => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        TableRowFlowClass      => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        TableCaptionFlowClass  => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
        TableCellFlowClass     => flow.as_block() as &'a mut layout::ftl_layout::FtlNode,
    }
}
