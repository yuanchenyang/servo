use std::cast;
use layout;
use layout::model::MaybeAuto::;
use layout::flow::{Flow, BlockFlowClass,InlineFlowClass};
use servo_util::geometry::Au;
use geom::{Point2D, Rect, Size2D};
use style::computed_values::{LengthOrPercentageOrAuto, LPA_Auto};

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
