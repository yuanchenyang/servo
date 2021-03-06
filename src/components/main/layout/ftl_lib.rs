use std::cast;
use std::mem;
use std::vec::MoveItems;
use layout;
use layout::model::MaybeAuto::;
use layout::fragment::{Fragment, SplitInfo, ScannedTextFragmentInfo,
                       ScannedTextFragment};
use layout::inline::InlineFragments;
use collections::{Deque, RingBuf};
use layout::flow::{Flow, BlockFlowClass,InlineFlowClass,TableWrapperFlowClass,
                   TableFlowClass,TableColGroupFlowClass,TableRowGroupFlowClass,
                   TableRowFlowClass,TableCaptionFlowClass,TableCellFlowClass};
use layout::util::ToGfxColor;

use geom::approxeq::ApproxEq;

use servo_util::geometry::Au;
use gfx::text::glyph::CharIndex;
use geom::{Point2D, Rect, Size2D, SideOffsets2D};
use style::computed_values::{LengthOrPercentageOrAuto, LPA_Auto};
use gfx::display_list::{DisplayList, BaseDisplayItem,
                        BorderDisplayItem, BorderDisplayItemClass};
use gfx::display_list::{SolidColorDisplayItem, SolidColorDisplayItemClass,
                        BackgroundAndBordersStackingLevel, ContentStackingLevel};
use gfx::display_list::{TextDecorations, TextDisplayItem, TextDisplayItemClass};

use gfx::display_list::OpaqueNode;
use layout::fragment::SpecificFragmentInfo;
use style::ComputedValues;
use sync::Arc;

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

pub fn spec_or_zero(length: LengthOrPercentageOrAuto, containing: Au) -> Au {
    //for b in flowbox.iter() {
    MaybeAuto::from_style(length, containing).specified_or_zero()
    //}
    //Au(0)
}

pub fn make_rect(x: Au, y: Au, width: Au, height: Au) -> Rect<Au> {
    Rect(Point2D(x, y),
         Size2D(width, height))
}

pub fn is_auto(length : LengthOrPercentageOrAuto) -> bool {
    match length {
        LPA_Auto => true,
        _ => false
    }
}

pub fn max (a : Au, b : Au) -> Au {
    match (a > b) {
        true  => a,
        false => b,
    }
}

pub fn new_display_list() -> FTLDisplayList {
    Some(DisplayList::new())
}
pub fn empty_display_list() -> DisplayList {
    DisplayList::new()
}

pub fn to_display_list(list: FTLDisplayList) -> DisplayList {
    list.unwrap()
}

/*pub fn merge_lists(parent: &mut DisplayList,
                  child:  &mut DisplayList) -> int {
    parent.push_all_move(mem::replace(child, DisplayList::new()));
    1
}*/

pub trait Optionable {
    fn to_option(self) -> Option<Self>;
}

impl Optionable for DisplayList {
    fn to_option(self) -> Option<DisplayList> {
        Some(self)
    }
}

pub type FTLDisplayList = Option<DisplayList>;
pub fn merge_lists(one: FTLDisplayList, two: FTLDisplayList) -> FTLDisplayList {
    let mut ret = one.unwrap();
    ret.push_all_move(two.unwrap());
    Some(ret)
}

pub fn add_border(list: FTLDisplayList, frag: &Fragment,
                 x: Au, y: Au, width: Au, height: Au,
                 t: Au, r: Au, b: Au, l: Au) -> FTLDisplayList {

    let list = list.unwrap();
    let style = frag.style();

    let top_color = style.resolve_color(style.get_border().border_top_color);
    let right_color = style.resolve_color(style.get_border().border_right_color);
    let bottom_color = style.resolve_color(style.get_border().border_bottom_color);
    let left_color = style.resolve_color(style.get_border().border_left_color);

    let border = SideOffsets2D::new(t, r, b, l);
    let border_display_item = box BorderDisplayItem {
        base: BaseDisplayItem::new(make_rect(x, y, width, height),
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
    let mut list = list;
    list.push(BorderDisplayItemClass(border_display_item));
    Some(list)
}

pub fn add_background(list: FTLDisplayList, frag: &Fragment,
                     x: Au, y: Au, width: Au, height: Au) -> FTLDisplayList {
    let list = list.unwrap();
    let style = frag.style();
    let background_color = style.resolve_color(style.get_background().background_color);
    let mut list = list;
    if !background_color.alpha.approx_eq(&0.0) {
        let solid_color_display_item = box SolidColorDisplayItem {
            base: BaseDisplayItem::new(make_rect(x, y, width, height),
                                       frag.node,
                                       BackgroundAndBordersStackingLevel),
            color: background_color.to_gfx_color(),
        };
        list.push(SolidColorDisplayItemClass(solid_color_display_item));
    }
    Some(list)
}

pub fn add_text_fragment(list: FTLDisplayList,
                         specific: SpecificFragmentInfo,
                         style: Arc<ComputedValues>,
                         node: OpaqueNode,
                         x: Au, y: Au, width: Au, height: Au) -> FTLDisplayList {
    let style = style.deref();
    let mut list = list.unwrap();
    match specific {
        ScannedTextFragment(ref text_fragment) => {
            // Compute text color.
            let text_color = style.get_color().color.to_gfx_color();

            // Compute text decorations.
            let text_decorations_in_effect = style
                .get_inheritedtext()
                ._servo_text_decorations_in_effect;

            let text_decorations = TextDecorations {
                underline: text_decorations_in_effect.underline.map(|c| c.to_gfx_color()),
                overline: text_decorations_in_effect.overline.map(|c| c.to_gfx_color()),
                line_through: text_decorations_in_effect.line_through
                    .map(|c| c.to_gfx_color()),
            };

            let mut bounds = make_rect(x, y, width, height);

            //debug!("{}", text_fragment.range);

            // Create the text fragment.
            let text_display_item = box TextDisplayItem {
                base: BaseDisplayItem::new(bounds, node, ContentStackingLevel),
                text_run: text_fragment.run.clone(),
                range: text_fragment.range,
                text_color: text_color,
                text_decorations: text_decorations,
            };
            list.push(TextDisplayItemClass(text_display_item));
        },
        // fail!("Each fragment in inline flow should be a ScannedTextFragment?")
        _ => {},
    }
    Some(list)
}

pub fn rect_height(rect: Rect<Au>) -> Au {
    rect.size.height
}

pub fn rect_width(rect: Rect<Au>) -> Au {
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
    debug!("{:s}",logstr)
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

struct Spliterator {
    items: MoveItems<Fragment>,
    leftover: Vec<Fragment>,
    work_list: RingBuf<Fragment>,
}

pub trait Splittable {
    fn fragments_split_iter(&mut self) -> Spliterator;
    fn end_iter(&mut self, iter: Spliterator);

}

impl Splittable for InlineFragments {
    fn fragments_split_iter(&mut self) -> Spliterator {
        Spliterator {
            items: mem::replace(&mut self.fragments, Vec::new()).move_iter(),
            leftover: Vec::new(),
            work_list: RingBuf::new()
        }
    }

    fn end_iter(&mut self, iter: Spliterator) {
        mem::replace(&mut self.fragments, iter.leftover);
    }
}

impl Spliterator {
    pub fn split_to_width(&mut self, remaining_width: Au, starts_line: bool) -> Option<&mut Fragment> {
        let cur_fragment = if self.work_list.is_empty() {
            match self.items.next() {
                None => {
                    debug!("split_to_width: no text left to split");
                    return None;
                },
                Some(fragment) => {
                    fragment
                }
            }
        } else {
            let fragment = self.work_list.pop_front().unwrap();
            fragment
        };

        let split = cur_fragment.find_split_info_for_width(CharIndex(0), remaining_width, starts_line);

        let mut has_rest = false;
        let ret = match split.map(|(left, right, run)| {
            let split_fragment = |split: SplitInfo| {
                let info = ScannedTextFragmentInfo::new(run.clone(), split.range);
                let specific = ScannedTextFragment(info);
                let size = Size2D(split.width, cur_fragment.border_box.size.height);
                cur_fragment.transform(size, specific)
            };

            (left.map(|x| { split_fragment(x) }),
             right.map(|x| { split_fragment(x) }))
        }) {
            None => {
                debug!("split_to_width: could not split fragment");
                Some(cur_fragment)
            },
            Some((left,right)) => {
                match right {
                    None => {},
                    Some(frag) => { has_rest = true; self.work_list.push_front(frag); }
                }
                left
            }
        };

        match ret {
            None => { 
                debug!("split_to_width: no left fragment; recurring");
                self.split_to_width(remaining_width, starts_line)
            },
            Some(mut x) => {
                debug!("split_to_width: returning left fragment");
                x.ftl_attrs.mustendline = has_rest;
                self.leftover.push(x);
                unsafe { Some(cast::transmute_mut_lifetime(self.leftover.mut_last().unwrap())) }
            }
        }
    }
}

pub trait LineMetrics {
    fn get_ascent(&mut self) -> Au;
    fn get_descent(&mut self) -> Au;
    fn get_lineheight(&mut self) -> Au;
}

impl LineMetrics for Fragment {
    fn get_ascent(&mut self) -> Au {
        self.inline_metrics().ascent
    }
    fn get_descent(&mut self) -> Au {
        Au(0)
    }
    fn get_lineheight(&mut self) -> Au {
        self.border_box.size.height
    }
}
