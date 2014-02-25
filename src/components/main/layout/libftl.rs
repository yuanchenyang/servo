use std::hashmap::HashMap;
use std::util;

use layout::flow::{Flow, FlowLeafSet, ImmutableFlowUtils,
                   MutableFlowUtils, MutableOwnedFlowUtils, base, mut_base};
use layout::flow::{PreorderFlowTraversal, PostorderFlowTraversal};


pub enum NodeType {
    top,
    midnode,
    leaf,
}

#[deriving(Clone)]
pub struct PrintInfoTraversal;

impl PreorderFlowTraversal for PrintInfoTraversal {
    #[inline]
    fn process(&mut self, flow: &mut Flow) -> bool {
        println!("{}", base(flow).id);
        true
    }
}

pub fn setFlowAttr<'a>(node: &'a mut Flow, attr: &'a str, value: int) {
    println!("ID {}: Set attr: {:s} to {:d}", base(node).id, attr, value)
}

pub fn getFlowAttr<'a>(node: &'a Flow, attr: &'a str) -> int{
    println!("ID {}: Get attr: {:s}", base(node).id, attr);
    1
}

pub fn isEven( num: int ) -> bool {
    num % 2 == 0
}

pub fn log(logstr: &str){
    //println!("{:s}",logstr)
}

pub fn with_kids<'a>(node: &'a mut Flow, func: |&mut Flow, &mut Flow|) {
    // for child in child_iter(node) {
    //     func(&mut node, &mut child);
    // }
    println!("with-kids!")
}

/*
pub fn inherit<'a>(visit: |&'a mut Flow|,node: &'a mut Flow) {
    visit(node);
    for child in child_iter(node) {
        inherit(|node| visit(node), child);
    }
}

pub fn synthesize<'a>(visit: |&'a mut Flow|, node: &'a mut Flow) {
    for child in child_iter(node) {
        synthesize(|node| visit(node), child);
    }
    visit(node);
}
*/
