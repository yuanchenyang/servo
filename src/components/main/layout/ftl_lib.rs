use std::hashmap::HashMap;
use std::util;
use std::cast;
use layout;
use layout::model::MaybeAuto::;
use layout::box_::Box;
use layout::flow::{Flow, BlockFlowClass,InlineFlowClass};
use servo_util::geometry::Au;
use extra::arc::Arc;
use geom::{Point2D, Rect, SideOffsets2D, Size2D};
use style::{ComputedValues};
use style::computed_values::{LengthOrPercentageOrAuto, LPA_Auto};

pub fn isEven( num: int ) -> bool {
    num % 2 == 0
}

pub fn base<'a,I>(node: &'a mut layout::ftl_layout::FtlNode) -> &'a mut I {
    unsafe {
        let (_, ptr) : (uint, uint) = cast::transmute(node);
        cast::transmute(ptr)
    }
}

pub fn getHeight(length: LengthOrPercentageOrAuto) -> Au {
    //for b in flowbox.iter() {
    MaybeAuto::from_style(length, Au::new(0)).specified_or_zero()
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
