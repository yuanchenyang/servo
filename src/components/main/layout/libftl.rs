use std::hashmap::HashMap;
use std::util;
use std::cast;
use layout::model::MaybeAuto::;
use layout::flow::{Flow, BlockFlowClass,InlineFlowClass};
use servo_util::geometry::Au;
use extra::arc::Arc;
use geom::{Point2D, Rect, SideOffsets2D, Size2D};
use style::{ComputedValues};

//use layout::flow::{Flow, ImmutableFlowUtils,
//                   MutableFlowUtils, MutableOwnedFlowUtils, base, mut_base,child_iter};
//use layout::flow::{PreorderFlowTraversal, PostorderFlowTraversal};


pub fn isEven( num: int ) -> bool {
    num % 2 == 0
}

pub fn base<'a,I>(node: &'a mut FtlNode) -> &'a mut I {
    unsafe {
        let (_, ptr) : (uint, uint) = cast::transmute(node);
        cast::transmute(ptr)
    }
}

// pub fn getAttr(node: BaseFlow, attr: &str) -> Au {
//
// }

pub fn getHeight(cv: &ComputedValues) -> Au {
    MaybeAuto::from_style(cv.Box.get().height,
                          Au::new(0)).specified_or_zero()
}

pub fn makeRect(x: Au, y: Au, width: Au, height: Au) -> Rect<Au> {
    Rect(Point2D(x, y),
         Size2D(width, height))
}

pub fn rectHeight(rect: Rect<Au>) -> Au {
    rect.size.height
}

pub fn inherit(visit: |&mut FtlNode|, node: &mut FtlNode) {
    visit(node);
    node.with_all_children(|child| {
        inherit(|node: &mut FtlNode| visit(node), child);
    });
}

pub fn synthesize(visit: |&mut FtlNode|,node: &mut FtlNode) {
    node.with_all_children(|child| {
        synthesize(|node: &mut FtlNode| visit(node), child);
    });
    visit(node);
}

pub fn log(logstr: &str){
    println!("{:s}",logstr)
}

pub trait FtlNode {
    fn with_all_children(&mut self, func: |&mut FtlNode|);
    fn visit_0(&mut self);
    fn visit_1(&mut self);
    fn visit_2(&mut self);
}

pub fn as_ftl_node<'a>(flow: &'a mut Flow) -> &'a mut FtlNode {
    match flow.class() {
        BlockFlowClass => flow.as_block() as &'a mut FtlNode,
        InlineFlowClass => flow.as_inline() as &'a mut FtlNode,
    }
}
