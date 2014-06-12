#[feature(globs)]
use layout::ftl_lib::*;
use layout::block::BlockFlow;
use layout::inline::InlineFlow;
use layout::table_colgroup::TableColGroupFlow;
use layout::flow::{mut_base,BaseFlow};
use layout::fragment::Fragment;
use layout::model::{specified};
use style::computed_values::{LengthOrPercentageOrAuto,LengthOrPercentage};
use servo_util::geometry::Au;
use gfx::display_list::{DisplayList};

pub trait FtlNode {
  fn with_all_children(&mut self, func: |&mut FtlNode|);
  fn visit_0(&mut self);
  fn visit_1(&mut self);
  fn visit_2(&mut self);
  fn visit_3(&mut self);
}

pub struct BaseFlowFtlAttrs {
  pub flowx: Au,
  pub flowy: Au,
  pub totalwidth: Au,
  pub makelists: int,
  pub flowheight: Au,
  pub availablewidth: Au,
  pub display_list: DisplayList,
  pub render: int,
  pub flowwidth: Au,
  pub containingx: Au,
  pub containingy: Au,
  pub totalheight: Au,
  pub bottom: Au,
  pub absy: Au,
  pub right: Au,
  pub absx: Au,
}

impl BaseFlowFtlAttrs {
  #[inline]
  pub fn new() ->BaseFlowFtlAttrs {
    BaseFlowFtlAttrs {
      flowx: Au::new(0),
      flowy: Au::new(0),
      totalwidth: Au::new(0),
      makelists: 0,
      flowheight: Au::new(0),
      availablewidth: Au::new(0),
      display_list: DisplayList::new(),
      render: 0,
      flowwidth: Au::new(0),
      containingx: Au::new(0),
      containingy: Au::new(0),
      totalheight: Au::new(0),
      bottom: Au::new(0),
      absy: Au::new(0),
      right: Au::new(0),
      absx: Au::new(0),
    }
  }
}

#[deriving(Clone)]
pub struct InlineBoxFtlAttrs {
  pub baselinefinal: Au,
  pub lineposy: Au,
  pub availabletextwidth: Au,
  pub posy: Au,
  pub lineheight: Au,
  pub posx: Au,
  pub endofline: bool,
  pub baseline: Au,
  pub right: Au,
}

impl InlineBoxFtlAttrs {
  #[inline]
  pub fn new() ->InlineBoxFtlAttrs {
    InlineBoxFtlAttrs {
      baselinefinal: Au::new(0),
      lineposy: Au::new(0),
      availabletextwidth: Au::new(0),
      posy: Au::new(0),
      lineheight: Au::new(0),
      posx: Au::new(0),
      endofline: false,
      baseline: Au::new(0),
      right: Au::new(0),
    }
  }
}

#[deriving(Clone)]
pub struct BlockFlowFtlAttrs {
  pub mr: Au,
  pub mbpvert: Au,
  pub childsheight: Au,
  pub mbphoriz: Au,
  pub mt: Au,
  pub br: Au,
  pub selfintrinsheight: Au,
  pub bt: Au,
  pub computedwidth: Au,
  pub bl: Au,
  pub pb: Au,
  pub mb: Au,
  pub pt: Au,
  pub pl: Au,
  pub childswidth: Au,
  pub bb: Au,
  pub ml: Au,
  pub pr: Au,
  pub selfintrinswidth: Au,
}

impl BlockFlowFtlAttrs {
  #[inline]
  pub fn new() ->BlockFlowFtlAttrs {
    BlockFlowFtlAttrs {
      mr: Au::new(0),
      mbpvert: Au::new(0),
      childsheight: Au::new(0),
      mbphoriz: Au::new(0),
      mt: Au::new(0),
      br: Au::new(0),
      selfintrinsheight: Au::new(0),
      bt: Au::new(0),
      computedwidth: Au::new(0),
      bl: Au::new(0),
      pb: Au::new(0),
      mb: Au::new(0),
      pt: Au::new(0),
      pl: Au::new(0),
      childswidth: Au::new(0),
      bb: Au::new(0),
      ml: Au::new(0),
      pr: Au::new(0),
      selfintrinswidth: Au::new(0),
    }
  }
}

//@type action
fn inlineflow_flowY ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn inlineflow_flowWidth ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn inlineflow_flowX ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn inlineflow_totalHeight ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn inlineflow_totalWidth ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_selfIntrinsWidth ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentageOrAuto) -> Au { spec_or_zero(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_totalWidth ( _ale_arg0: Au,  _ale_arg2: Au,  _ale_arg1: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 }
//@type action
fn blockflow_br ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_pt ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_bl ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_flowX ( _ale_arg0: Au,  _ale_arg1: Au) -> Au { _ale_arg0 + _ale_arg1 }
//@type action
fn blockflow_ml ( _ale_arg3: Au,  _ale_arg1: LengthOrPercentageOrAuto,  _ale_arg0: LengthOrPercentageOrAuto,  _ale_arg6: Au,  _ale_arg2: LengthOrPercentageOrAuto,  _ale_arg5: Au,  _ale_arg8: Au,  _ale_arg7: Au,  _ale_arg4: Au) -> Au { if (is_auto(_ale_arg0)) { (if (is_auto(_ale_arg1)) { Au(0) } else { (if (is_auto(_ale_arg2)) { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8) / Au(2) } else { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8 - spec_or_zero(_ale_arg2, _ale_arg3)) }) }) } else { spec_or_zero(_ale_arg0, _ale_arg3) } }
//@type action
fn blockflow_bb ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_selfIntrinsHeight ( _ale_arg0: LengthOrPercentageOrAuto) -> Au { spec_or_zero(_ale_arg0, Au(0)) }
//@type action
fn blockflow_mbpVert ( _ale_arg3: Au,  _ale_arg0: Au,  _ale_arg1: Au,  _ale_arg2: Au,  _ale_arg5: Au,  _ale_arg4: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 }
//@type action
fn blockflow_mr ( _ale_arg3: Au,  _ale_arg2: LengthOrPercentageOrAuto,  _ale_arg1: LengthOrPercentageOrAuto,  _ale_arg7: Au,  _ale_arg0: LengthOrPercentageOrAuto,  _ale_arg5: Au,  _ale_arg8: Au,  _ale_arg6: Au,  _ale_arg4: Au) -> Au { if ((((! is_auto(_ale_arg0)) && ((is_auto(_ale_arg1) || is_auto(_ale_arg2)))))) { spec_or_zero(_ale_arg0, _ale_arg3) } else { (if (is_auto(_ale_arg1)) { Au(0) } else { (if (is_auto(_ale_arg2)) { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8) / Au(2) } else { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8 - spec_or_zero(_ale_arg2, _ale_arg3)) }) }) } }
//@type action
fn blockflow_pr ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_bt ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_flowY ( _ale_arg0: Au,  _ale_arg1: Au) -> Au { _ale_arg0 + _ale_arg1 }
//@type action
fn blockflow_mb ( _ale_arg0: LengthOrPercentageOrAuto,  _ale_arg1: Au) -> Au { if (is_auto(_ale_arg0)) { Au(0) } else { spec_or_zero(_ale_arg0, _ale_arg1) } }
//@type action
fn blockflow_mbpHoriz ( _ale_arg1: Au,  _ale_arg4: Au,  _ale_arg2: Au,  _ale_arg5: Au,  _ale_arg0: Au,  _ale_arg3: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 }
//@type action
fn blockflow_mt ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentageOrAuto) -> Au { if (is_auto(_ale_arg0)) { Au(0) } else { spec_or_zero(_ale_arg0, _ale_arg1) } }
//@type action
fn blockflow_flowHeight ( _ale_arg2: Au,  _ale_arg1: Au,  _ale_arg3: Au,  _ale_arg0: Au,  _ale_arg4: Au,  _ale_arg5: Au) -> Au { if ((_ale_arg0 == Au(0))) { _ale_arg1 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 } else { _ale_arg0 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 } }
//@type action
fn blockflow_totalHeight ( _ale_arg0: Au,  _ale_arg1: Au,  _ale_arg2: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 }
//@type action
fn blockflow_computedWidth ( _ale_arg0: bool,  _ale_arg3: Au,  _ale_arg2: LengthOrPercentageOrAuto,  _ale_arg4: Au,  _ale_arg5: Au,  _ale_arg1: Au) -> Au { if (_ale_arg0) { _ale_arg1 } else { (if (is_auto(_ale_arg2)) { _ale_arg3 - _ale_arg4 } else { _ale_arg5 }) } }
//@type action
fn blockflow_flowWidth ( _ale_arg0: bool,  _ale_arg5: Au,  _ale_arg3: Au,  _ale_arg2: Au,  _ale_arg6: Au,  _ale_arg1: Au,  _ale_arg4: Au) -> Au { if (_ale_arg0) { _ale_arg1 } else { _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 + _ale_arg6 } }
//@type action
fn blockflow_pl ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_display_list ( _ale_arg6: Au,  _ale_arg5: Au,  _ale_arg10: Au,  _ale_arg4: Au,  _ale_arg3: Au,  _ale_arg1: Au,  _ale_arg9: Au,  _ale_arg8: Au,  _ale_arg2: Au,  _ale_arg0: &Fragment,  _ale_arg7: Au) -> DisplayList { add_border(add_background(new_display_list(), _ale_arg0, _ale_arg1 + _ale_arg2, _ale_arg3 + _ale_arg4, _ale_arg5, _ale_arg6), _ale_arg0, _ale_arg1 + _ale_arg2, _ale_arg3 + _ale_arg4, _ale_arg5, _ale_arg6, _ale_arg7, _ale_arg8, _ale_arg9, _ale_arg10) }
//@type action
fn blockflow_pb ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn tablecolgroupflow_flowY () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowX () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_totalWidth () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowWidth () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowHeight () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_totalHeight () -> Au { Au(0) }
impl FtlNode for TableColGroupFlow {
 fn with_all_children(&mut self, func: |&mut FtlNode|) {
 }
 fn visit_0(&mut self) {
  self.base.position.size.height = (tablecolgroupflow_flowHeight());
  self.base.position.size.width = (tablecolgroupflow_flowWidth());
  self.base.position.origin.x = (tablecolgroupflow_flowX());
  self.base.position.origin.y = (tablecolgroupflow_flowY());
  self.base.ftl_attrs.totalheight = (tablecolgroupflow_totalHeight());
  self.base.ftl_attrs.totalwidth = (tablecolgroupflow_totalWidth());
 
 }
 fn visit_1(&mut self) {
 
 }
 fn visit_2(&mut self) {
 
 }
 fn visit_3(&mut self) {
 
 }
}
impl FtlNode for InlineFlow {
 fn with_all_children(&mut self, func: |&mut FtlNode|) {
 }
 fn visit_0(&mut self) {
  self.base.position.size.width = (inlineflow_flowWidth(self.base.ftl_attrs.availablewidth));
  self.base.ftl_attrs.totalwidth = (inlineflow_totalWidth(self.base.position.size.width));

  { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
    first = false;
    old_child = Some(child);
  }

  }


  let text_availabletextwidth_init = ((self.base.ftl_attrs.availablewidth ));
  let mut text_availabletextwidth_last = (text_availabletextwidth_init);
    let text_baseline_init = ((Au(0)));
  let mut text_baseline_last = (text_baseline_init);
    let text_endofline_init = ((false));
  let mut text_endofline_last = (text_endofline_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments_split_iter();
  let mut first = true;
  loop {
    let child = match children.split_to_width((if first { text_availabletextwidth_init } else { old_child.get_ref().ftl_attrs.availabletextwidth }), (if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { None => {break;} Some(c) => {c} };
      child.ftl_attrs.availabletextwidth = ((if (((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline }))) { ((if first { text_availabletextwidth_init } else { old_child.get_ref().ftl_attrs.availabletextwidth }) - child.border_box.size.width) } else { (self.base.ftl_attrs.availablewidth) }));
      text_availabletextwidth_last = child.ftl_attrs.availabletextwidth;
      child.ftl_attrs.baseline = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { Au(0) } else { max((if first { text_baseline_init } else { old_child.get_ref().ftl_attrs.baseline }), child.get_ascent()) }));
      text_baseline_last = child.ftl_attrs.baseline;
      child.ftl_attrs.endofline = (((child.border_box.size.width > (if first { text_availabletextwidth_init } else { old_child.get_ref().ftl_attrs.availabletextwidth }))));
      text_endofline_last = child.ftl_attrs.endofline;
    first = false;
    old_child = Some(child);
  }
self.fragments.end_iter(children);
  }


  let text_baselinefinal_init = ((Au(0)));
  let mut text_baselinefinal_last = (text_baselinefinal_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.baselinefinal = ((if (child.ftl_attrs.endofline) { child.ftl_attrs.baseline } else { (if first { text_baselinefinal_init } else { old_child.get_ref().ftl_attrs.baselinefinal }) }));
      text_baselinefinal_last = child.ftl_attrs.baselinefinal;
    first = false;
    old_child = Some(child);
  }

  }


  let text_lineheight_init = ((Au(0)));
  let mut text_lineheight_last = (text_lineheight_init);
    let text_lineposy_init = ((Au(0)));
  let mut text_lineposy_last = (text_lineposy_init);
    let text_right_init = ((Au(0)));
  let mut text_right_last = (text_right_init);
    let flowheight_init = ((Au(0)));
  self.base.position.size.height = (flowheight_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.lineheight = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { child.get_lineheight() } else { max((if first { text_lineheight_init } else { old_child.get_ref().ftl_attrs.lineheight }), child.get_lineheight()) }));
      text_lineheight_last = child.ftl_attrs.lineheight;
      child.ftl_attrs.lineposy = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { (if first { text_lineposy_init } else { old_child.get_ref().ftl_attrs.lineposy }) } else { ((if first { text_lineposy_init } else { old_child.get_ref().ftl_attrs.lineposy }) + (if first { text_lineheight_init } else { old_child.get_ref().ftl_attrs.lineheight })) }));
      text_lineposy_last = child.ftl_attrs.lineposy;
      child.ftl_attrs.right = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { child.border_box.size.width } else { (if first { text_right_init } else { old_child.get_ref().ftl_attrs.right }) + child.border_box.size.width }));
      text_right_last = child.ftl_attrs.right;
      self.base.position.size.height = ((child.ftl_attrs.lineheight + self.base.position.size.height ));
    first = false;
    old_child = Some(child);
  }

  }

  self.base.ftl_attrs.totalheight = (inlineflow_totalHeight(self.base.position.size.height));
 
 }
 fn visit_1(&mut self) {
 
 }
 fn visit_2(&mut self) {
  self.base.position.origin.x = (inlineflow_flowX(self.base.ftl_attrs.containingx));
  self.base.position.origin.y = (inlineflow_flowY(self.base.ftl_attrs.containingy));

  let text_posx_init = ((Au(0)));
  let mut text_posx_last = (text_posx_init);
    let text_posy_init = ((Au(0)));
  let mut text_posy_last = (text_posy_init);
    let render_init = ((0));
  self.base.ftl_attrs.render = (render_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.posx = ((self.base.ftl_attrs.absx ));
      text_posx_last = child.ftl_attrs.posx;
      child.ftl_attrs.posy = ((self.base.ftl_attrs.absy + child.ftl_attrs.lineposy + child.ftl_attrs.baselinefinal - child.get_ascent() ));
      text_posy_last = child.ftl_attrs.posy;
      self.base.ftl_attrs.render = ((add_text_fragment(&mut self.base.display_list, child.specific.clone(), child.style.clone(), child.node.clone(), child.ftl_attrs.posx, child.ftl_attrs.posy, child.ftl_attrs.availabletextwidth, child.ftl_attrs.lineheight)));
    first = false;
    old_child = Some(child);
  }

  }

 
 }
 fn visit_3(&mut self) {
 
 }
}
impl FtlNode for BlockFlow {
 fn with_all_children(&mut self, func: |&mut FtlNode|) {
  for child in self.base.children.mut_iter() {
    func(as_ftl_node(child));
  }
 }
 fn visit_0(&mut self) {
  self.ftl_attrs.bb = (blockflow_bb(self.fragment.style().get_border().border_bottom_width));
  self.ftl_attrs.bl = (blockflow_bl(self.fragment.style().get_border().border_left_width));
  self.ftl_attrs.br = (blockflow_br(self.fragment.style().get_border().border_right_width));
  self.ftl_attrs.bt = (blockflow_bt(self.fragment.style().get_border().border_top_width));
  self.ftl_attrs.mb = (blockflow_mb(self.fragment.style().get_margin().margin_bottom, self.base.ftl_attrs.availablewidth));
  self.ftl_attrs.mt = (blockflow_mt(self.base.ftl_attrs.availablewidth, self.fragment.style().get_margin().margin_top));
  self.ftl_attrs.pb = (blockflow_pb(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_bottom));
  self.ftl_attrs.pl = (blockflow_pl(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_left));
  self.ftl_attrs.pr = (blockflow_pr(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_right));
  self.ftl_attrs.pt = (blockflow_pt(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_top));
  self.ftl_attrs.selfintrinsheight = (blockflow_selfIntrinsHeight(self.fragment.style().get_box().height));
  self.ftl_attrs.selfintrinswidth = (blockflow_selfIntrinsWidth(self.base.ftl_attrs.availablewidth, self.fragment.style().get_box().width));
  self.ftl_attrs.mbpvert = (blockflow_mbpVert(self.ftl_attrs.pb, self.ftl_attrs.mt, self.ftl_attrs.mb, self.ftl_attrs.pt, self.ftl_attrs.bb, self.ftl_attrs.bt));
  self.ftl_attrs.ml = (blockflow_ml(self.base.ftl_attrs.availablewidth, self.fragment.style().get_box().width, self.fragment.style().get_margin().margin_left, self.ftl_attrs.bl, self.fragment.style().get_margin().margin_right, self.ftl_attrs.pl, self.ftl_attrs.selfintrinswidth, self.ftl_attrs.br, self.ftl_attrs.pr));
  self.ftl_attrs.mr = (blockflow_mr(self.base.ftl_attrs.availablewidth, self.fragment.style().get_margin().margin_left, self.fragment.style().get_box().width, self.ftl_attrs.bl, self.fragment.style().get_margin().margin_right, self.ftl_attrs.pl, self.ftl_attrs.selfintrinswidth, self.ftl_attrs.br, self.ftl_attrs.pr));
  self.ftl_attrs.mbphoriz = (blockflow_mbpHoriz(self.ftl_attrs.mr, self.ftl_attrs.bl, self.ftl_attrs.pl, self.ftl_attrs.br, self.ftl_attrs.ml, self.ftl_attrs.pr));
  self.ftl_attrs.computedwidth = (blockflow_computedWidth(self.is_root, self.base.ftl_attrs.availablewidth, self.fragment.style().get_box().width, self.ftl_attrs.mbphoriz, self.ftl_attrs.selfintrinswidth, self.screenwidth));
  self.base.position.size.width = (blockflow_flowWidth(self.is_root, self.ftl_attrs.bl, self.ftl_attrs.pl, self.ftl_attrs.computedwidth, self.ftl_attrs.br, self.screenwidth, self.ftl_attrs.pr));
  self.base.ftl_attrs.totalwidth = (blockflow_totalWidth(self.base.position.size.width, self.ftl_attrs.mr, self.ftl_attrs.ml));

  let flowchildren_availablewidth_init = ((Au(0)));
  let mut flowchildren_availablewidth_last = (flowchildren_availablewidth_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.availablewidth = ((self.ftl_attrs.computedwidth ));
      flowchildren_availablewidth_last = child.ftl_attrs.availablewidth;
    first = false;
    old_child = Some(child);
  }

  }

 
 }
 fn visit_1(&mut self) {

  { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
    first = false;
    old_child = Some(child);
  }

  }


  let childsheight_init = ((Au(0)));
  self.ftl_attrs.childsheight = (childsheight_init);
    let childswidth_init = ((Au(0)));
  self.ftl_attrs.childswidth = (childswidth_init);
    let flowchildren_bottom_init = ((self.ftl_attrs.pt + self.ftl_attrs.bt ));
  let mut flowchildren_bottom_last = (flowchildren_bottom_init);
    let flowchildren_right_init = ((Au(0)));
  let mut flowchildren_right_last = (flowchildren_right_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      self.ftl_attrs.childsheight = (((self.ftl_attrs.childsheight + child.ftl_attrs.totalheight)));
      self.ftl_attrs.childswidth = ((max(self.ftl_attrs.childswidth, child.ftl_attrs.totalwidth)));
      child.ftl_attrs.bottom = ((((if first { flowchildren_bottom_init } else { old_child.get_ref().ftl_attrs.bottom }) + child.ftl_attrs.totalheight)));
      flowchildren_bottom_last = child.ftl_attrs.bottom;
      child.ftl_attrs.right = (((child.ftl_attrs.totalwidth + self.ftl_attrs.pl + self.ftl_attrs.bl)));
      flowchildren_right_last = child.ftl_attrs.right;
    first = false;
    old_child = Some(child);
  }

  }


  let flowchildren_containingx_init = ((Au(0)));
  let mut flowchildren_containingx_last = (flowchildren_containingx_init);
    let flowchildren_containingy_init = ((Au(0)));
  let mut flowchildren_containingy_last = (flowchildren_containingy_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.containingx = ((child.ftl_attrs.right - child.ftl_attrs.totalwidth ));
      flowchildren_containingx_last = child.ftl_attrs.containingx;
      child.ftl_attrs.containingy = ((child.ftl_attrs.bottom - child.ftl_attrs.totalheight ));
      flowchildren_containingy_last = child.ftl_attrs.containingy;
    first = false;
    old_child = Some(child);
  }

  }

  self.base.position.size.height = (blockflow_flowHeight(self.ftl_attrs.pb, self.ftl_attrs.childsheight, self.ftl_attrs.pt, self.ftl_attrs.selfintrinsheight, self.ftl_attrs.bb, self.ftl_attrs.bt));
  self.base.ftl_attrs.totalheight = (blockflow_totalHeight(self.base.position.size.height, self.ftl_attrs.mt, self.ftl_attrs.mb));
 
 }
 fn visit_2(&mut self) {
  self.base.display_list = (blockflow_display_list(self.base.position.size.height, self.base.position.size.width, self.ftl_attrs.bl, self.ftl_attrs.mt, self.base.ftl_attrs.absy, self.base.ftl_attrs.absx, self.ftl_attrs.bb, self.ftl_attrs.br, self.ftl_attrs.ml, &mut self.fragment, self.ftl_attrs.bt));
  self.base.position.origin.x = (blockflow_flowX(self.base.ftl_attrs.containingx, self.ftl_attrs.ml));
  self.base.position.origin.y = (blockflow_flowY(self.base.ftl_attrs.containingy, self.ftl_attrs.mt));

  let flowchildren_absx_init = ((Au(0)));
  let mut flowchildren_absx_last = (flowchildren_absx_init);
    let flowchildren_absy_init = ((Au(0)));
  let mut flowchildren_absy_last = (flowchildren_absy_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.absx = ((child.ftl_attrs.containingx + self.base.ftl_attrs.absx + self.ftl_attrs.ml ));
      flowchildren_absx_last = child.ftl_attrs.absx;
      child.ftl_attrs.absy = ((child.ftl_attrs.containingy + self.base.ftl_attrs.absy + self.ftl_attrs.mt ));
      flowchildren_absy_last = child.ftl_attrs.absy;
    first = false;
    old_child = Some(child);
  }

  }

 
 }
 fn visit_3(&mut self) {

  { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
    first = false;
    old_child = Some(child);
  }

  }


  let makelists_init = ((0));
  self.base.ftl_attrs.makelists = (makelists_init);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      self.base.ftl_attrs.makelists = ((merge_lists(&mut self.base.display_list, &mut child.display_list)));
    first = false;
    old_child = Some(child);
  }

  }

 
 }
}
pub fn layout (root: &mut FtlNode) {
  inherit(|node| node.visit_0(), root);
  synthesize(|node| node.visit_1(), root);
  inherit(|node| node.visit_2(), root);
  synthesize(|node| node.visit_3(), root);
}
