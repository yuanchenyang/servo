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
fn blockflow_mr ( _ale_arg3: Au,  _ale_arg2: LengthOrPercentageOrAuto,  _ale_arg1: LengthOrPercentageOrAuto,  _ale_arg7: Au,  _ale_arg0: LengthOrPercentageOrAuto,  _ale_arg5: Au,  _ale_arg8: Au,  _ale_arg6: Au,  _ale_arg4: Au) -> Au { if ((((! is_auto(_ale_arg0)) && ((is_auto(_ale_arg1) || is_auto(_ale_arg2)))))) { spec_or_zero(_ale_arg0, _ale_arg3) } else { (if (is_auto(_ale_arg1)) { Au(0) } else { (if (is_auto(_ale_arg2)) { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8) / Au(2) } else { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8 - spec_or_zero(_ale_arg2, _ale_arg3)) }) }) } }
//@type action
fn blockflow_selfIntrinsHeight ( _ale_arg0: LengthOrPercentageOrAuto) -> Au { spec_or_zero(_ale_arg0, Au(0)) }
//@type action
fn blockflow_pt ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_bt ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_flowWidth ( _ale_arg0: bool,  _ale_arg5: Au,  _ale_arg3: Au,  _ale_arg2: Au,  _ale_arg6: Au,  _ale_arg1: Au,  _ale_arg4: Au) -> Au { if (_ale_arg0) { _ale_arg1 } else { _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 + _ale_arg6 } }
//@type action
fn blockflow_ml ( _ale_arg3: Au,  _ale_arg1: LengthOrPercentageOrAuto,  _ale_arg0: LengthOrPercentageOrAuto,  _ale_arg6: Au,  _ale_arg2: LengthOrPercentageOrAuto,  _ale_arg5: Au,  _ale_arg8: Au,  _ale_arg7: Au,  _ale_arg4: Au) -> Au { if (is_auto(_ale_arg0)) { (if (is_auto(_ale_arg1)) { Au(0) } else { (if (is_auto(_ale_arg2)) { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8) / Au(2) } else { (_ale_arg3 - _ale_arg4 - _ale_arg5 - _ale_arg6 - _ale_arg7 - _ale_arg8 - spec_or_zero(_ale_arg2, _ale_arg3)) }) }) } else { spec_or_zero(_ale_arg0, _ale_arg3) } }
//@type action
fn blockflow_mbpHoriz ( _ale_arg1: Au,  _ale_arg4: Au,  _ale_arg2: Au,  _ale_arg5: Au,  _ale_arg0: Au,  _ale_arg3: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 }
//@type action
fn blockflow_mb ( _ale_arg0: LengthOrPercentageOrAuto,  _ale_arg1: Au) -> Au { if (is_auto(_ale_arg0)) { Au(0) } else { spec_or_zero(_ale_arg0, _ale_arg1) } }
//@type action
fn blockflow_pr ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_br ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_flowX ( _ale_arg0: Au,  _ale_arg1: Au) -> Au { _ale_arg0 + _ale_arg1 }
//@type action
fn blockflow_pb ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_mt ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentageOrAuto) -> Au { if (is_auto(_ale_arg0)) { Au(0) } else { spec_or_zero(_ale_arg0, _ale_arg1) } }
//@type action
fn blockflow_mbpVert ( _ale_arg3: Au,  _ale_arg0: Au,  _ale_arg1: Au,  _ale_arg2: Au,  _ale_arg5: Au,  _ale_arg4: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 }
//@type action
fn blockflow_totalHeight ( _ale_arg0: Au,  _ale_arg1: Au,  _ale_arg2: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 }
//@type action
fn blockflow_display_list ( _ale_arg6: Au,  _ale_arg5: Au,  _ale_arg10: Au,  _ale_arg4: Au,  _ale_arg3: Au,  _ale_arg1: Au,  _ale_arg9: Au,  _ale_arg8: Au,  _ale_arg2: Au,  _ale_arg0: &Fragment,  _ale_arg7: Au) -> DisplayList { add_border(add_background(new_display_list(), _ale_arg0, _ale_arg1 + _ale_arg2, _ale_arg3 + _ale_arg4, _ale_arg5, _ale_arg6), _ale_arg0, _ale_arg1 + _ale_arg2, _ale_arg3 + _ale_arg4, _ale_arg5, _ale_arg6, _ale_arg7, _ale_arg8, _ale_arg9, _ale_arg10) }
//@type action
fn blockflow_flowHeight ( _ale_arg2: Au,  _ale_arg1: Au,  _ale_arg3: Au,  _ale_arg0: Au,  _ale_arg4: Au,  _ale_arg5: Au) -> Au { if ((_ale_arg0 == Au(0))) { _ale_arg1 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 } else { _ale_arg0 + _ale_arg2 + _ale_arg3 + _ale_arg4 + _ale_arg5 } }
//@type action
fn blockflow_computedWidth ( _ale_arg0: bool,  _ale_arg3: Au,  _ale_arg2: LengthOrPercentageOrAuto,  _ale_arg4: Au,  _ale_arg5: Au,  _ale_arg1: Au) -> Au { if (_ale_arg0) { _ale_arg1 } else { (if (is_auto(_ale_arg2)) { _ale_arg3 - _ale_arg4 } else { _ale_arg5 }) } }
//@type action
fn blockflow_pl ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentage) -> Au { specified(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_bl ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn blockflow_flowY ( _ale_arg0: Au,  _ale_arg1: Au) -> Au { _ale_arg0 + _ale_arg1 }
//@type action
fn blockflow_totalWidth ( _ale_arg0: Au,  _ale_arg2: Au,  _ale_arg1: Au) -> Au { _ale_arg0 + _ale_arg1 + _ale_arg2 }
//@type action
fn blockflow_selfIntrinsWidth ( _ale_arg1: Au,  _ale_arg0: LengthOrPercentageOrAuto) -> Au { spec_or_zero(_ale_arg0, _ale_arg1) }
//@type action
fn blockflow_bb ( _ale_arg0: Au) -> Au { _ale_arg0 }
//@type action
fn inlineflow_flowHeight () -> Au { Au(0) }
//@type action
fn inlineflow_flowY () -> Au { Au(0) }
//@type action
fn inlineflow_flowWidth () -> Au { Au(0) }
//@type action
fn inlineflow_totalHeight () -> Au { Au(0) }
//@type action
fn inlineflow_totalWidth () -> Au { Au(0) }
//@type action
fn inlineflow_flowX () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowY () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowX () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowHeight () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_totalHeight () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_flowWidth () -> Au { Au(0) }
//@type action
fn tablecolgroupflow_totalWidth () -> Au { Au(0) }
impl FtlNode for InlineFlow {
 fn with_all_children(&mut self, func: |&mut FtlNode|) {
  for child in self.fragments.fragments.mut_iter() {
    // func(as_ftl_node(child));
  }
 }
 fn visit_0(&mut self) {
  debug!("FTL:   visit  InlineFlow {}", "0");
  self.base.position.size.height = (inlineflow_flowHeight());
  debug!("FTL:     inlineflow_flowHeight {}", self.base.position.size.height);
  self.base.position.size.width = (inlineflow_flowWidth());
  debug!("FTL:     inlineflow_flowWidth {}", self.base.position.size.width);
  self.base.position.origin.x = (inlineflow_flowX());
  debug!("FTL:     inlineflow_flowX {}", self.base.position.origin.x);
  self.base.position.origin.y = (inlineflow_flowY());
  debug!("FTL:     inlineflow_flowY {}", self.base.position.origin.y);
  self.base.ftl_attrs.totalheight = (inlineflow_totalHeight());
  debug!("FTL:     inlineflow_totalHeight {}", self.base.ftl_attrs.totalheight);
  self.base.ftl_attrs.totalwidth = (inlineflow_totalWidth());
  debug!("FTL:     inlineflow_totalWidth {}", self.base.ftl_attrs.totalwidth);

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
  debug!("FTL:       init text@availableTextWidth {}", text_availabletextwidth_init);
  debug!("FTL:     last init text_availabletextwidth_last {}", text_availabletextwidth_last);
    let text_baseline_init = ((Au(0)));
  let mut text_baseline_last = (text_baseline_init);
  debug!("FTL:       init text@baseline {}", text_baseline_init);
  debug!("FTL:     last init text_baseline_last {}", text_baseline_last);
    let text_endofline_init = ((false));
  let mut text_endofline_last = (text_endofline_init);
  debug!("FTL:       init text@endOfLine {}", text_endofline_init);
  debug!("FTL:     last init text_endofline_last {}", text_endofline_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments_split_iter();
  let mut first = true;
  loop {
    let child = match children.split_to_width((if first { text_availabletextwidth_init } else { old_child.get_ref().ftl_attrs.availabletextwidth }), (if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { None => {break;} Some(c) => {c} };
      child.ftl_attrs.availabletextwidth = ((if (((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline }))) { ((if first { text_availabletextwidth_init } else { old_child.get_ref().ftl_attrs.availabletextwidth }) - child.border_box.size.width) } else { (self.base.ftl_attrs.availablewidth) }));
      text_availabletextwidth_last = child.ftl_attrs.availabletextwidth;
      debug!("FTL:          step text@availableTextWidth {}", child.ftl_attrs.availabletextwidth);
      child.ftl_attrs.baseline = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { Au(0) } else { max((if first { text_baseline_init } else { old_child.get_ref().ftl_attrs.baseline }), child.get_ascent()) }));
      text_baseline_last = child.ftl_attrs.baseline;
      debug!("FTL:          step text@baseline {}", child.ftl_attrs.baseline);
      child.ftl_attrs.endofline = (((child.border_box.size.width > (if first { text_availabletextwidth_init } else { old_child.get_ref().ftl_attrs.availabletextwidth }))));
      text_endofline_last = child.ftl_attrs.endofline;
      debug!("FTL:          step text@endOfLine {}", child.ftl_attrs.endofline);
    first = false;
    old_child = Some(child);
  }
self.fragments.end_iter(children);
  }


  let text_baselinefinal_init = ((Au(0)));
  let mut text_baselinefinal_last = (text_baselinefinal_init);
  debug!("FTL:       init text@baselineFinal {}", text_baselinefinal_init);
  debug!("FTL:     last init text_baselinefinal_last {}", text_baselinefinal_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.baselinefinal = ((if (child.ftl_attrs.endofline) { child.ftl_attrs.baseline } else { (if first { text_baselinefinal_init } else { old_child.get_ref().ftl_attrs.baselinefinal }) }));
      text_baselinefinal_last = child.ftl_attrs.baselinefinal;
      debug!("FTL:          step text@baselineFinal {}", child.ftl_attrs.baselinefinal);
    first = false;
    old_child = Some(child);
  }

  }


  let text_lineheight_init = ((Au(0)));
  let mut text_lineheight_last = (text_lineheight_init);
  debug!("FTL:       init text@lineHeight {}", text_lineheight_init);
  debug!("FTL:     last init text_lineheight_last {}", text_lineheight_last);
    let text_lineposy_init = ((Au(0)));
  let mut text_lineposy_last = (text_lineposy_init);
  debug!("FTL:       init text@linePosY {}", text_lineposy_init);
  debug!("FTL:     last init text_lineposy_last {}", text_lineposy_last);
    let text_right_init = ((Au(0)));
  let mut text_right_last = (text_right_init);
  debug!("FTL:       init text@right {}", text_right_init);
  debug!("FTL:     last init text_right_last {}", text_right_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.lineheight = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { child.get_lineheight() } else { max((if first { text_lineheight_init } else { old_child.get_ref().ftl_attrs.lineheight }), child.get_lineheight()) }));
      text_lineheight_last = child.ftl_attrs.lineheight;
      debug!("FTL:          step text@lineHeight {}", child.ftl_attrs.lineheight);
      child.ftl_attrs.lineposy = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { ((if first { text_lineposy_init } else { old_child.get_ref().ftl_attrs.lineposy }) + (if first { text_lineheight_init } else { old_child.get_ref().ftl_attrs.lineheight })) } else { (if first { text_lineposy_init } else { old_child.get_ref().ftl_attrs.lineposy }) }));
      text_lineposy_last = child.ftl_attrs.lineposy;
      debug!("FTL:          step text@linePosY {}", child.ftl_attrs.lineposy);
      child.ftl_attrs.right = ((if ((if first { text_endofline_init } else { old_child.get_ref().ftl_attrs.endofline })) { child.border_box.size.width } else { (if first { text_right_init } else { old_child.get_ref().ftl_attrs.right }) + child.border_box.size.width }));
      text_right_last = child.ftl_attrs.right;
      debug!("FTL:          step text@right {}", child.ftl_attrs.right);
    first = false;
    old_child = Some(child);
  }

  }


  let text_posx_init = ((Au(0)));
  let mut text_posx_last = (text_posx_init);
  debug!("FTL:       init text@posX {}", text_posx_init);
  debug!("FTL:     last init text_posx_last {}", text_posx_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.posx = ((child.ftl_attrs.right - child.border_box.size.width ));
      text_posx_last = child.ftl_attrs.posx;
      debug!("FTL:          step text@posX {}", child.ftl_attrs.posx);
    first = false;
    old_child = Some(child);
  }

  }


  let text_posy_init = ((Au(0)));
  let mut text_posy_last = (text_posy_init);
  debug!("FTL:       init text@posY {}", text_posy_init);
  debug!("FTL:     last init text_posy_last {}", text_posy_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut Fragment> = None;
  let mut children = self.fragments.fragments.mut_iter();
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.posy = ((child.ftl_attrs.lineposy + child.ftl_attrs.baselinefinal - child.get_ascent() ));
      text_posy_last = child.ftl_attrs.posy;
      debug!("FTL:          step text@posY {}", child.ftl_attrs.posy);
    first = false;
    old_child = Some(child);
  }

  }


 }
 fn visit_1(&mut self) {
  debug!("FTL:   visit  InlineFlow {}", "1");

 }
 fn visit_2(&mut self) {
  debug!("FTL:   visit  InlineFlow {}", "2");

 }
 fn visit_3(&mut self) {
  debug!("FTL:   visit  InlineFlow {}", "3");

 }
}
impl FtlNode for TableColGroupFlow {
 fn with_all_children(&mut self, func: |&mut FtlNode|) {
 }
 fn visit_0(&mut self) {
  debug!("FTL:   visit  TableColGroupFlow {}", "0");
  self.base.position.size.height = (tablecolgroupflow_flowHeight());
  debug!("FTL:     tablecolgroupflow_flowHeight {}", self.base.position.size.height);
  self.base.position.size.width = (tablecolgroupflow_flowWidth());
  debug!("FTL:     tablecolgroupflow_flowWidth {}", self.base.position.size.width);
  self.base.position.origin.x = (tablecolgroupflow_flowX());
  debug!("FTL:     tablecolgroupflow_flowX {}", self.base.position.origin.x);
  self.base.position.origin.y = (tablecolgroupflow_flowY());
  debug!("FTL:     tablecolgroupflow_flowY {}", self.base.position.origin.y);
  self.base.ftl_attrs.totalheight = (tablecolgroupflow_totalHeight());
  debug!("FTL:     tablecolgroupflow_totalHeight {}", self.base.ftl_attrs.totalheight);
  self.base.ftl_attrs.totalwidth = (tablecolgroupflow_totalWidth());
  debug!("FTL:     tablecolgroupflow_totalWidth {}", self.base.ftl_attrs.totalwidth);

 }
 fn visit_1(&mut self) {
  debug!("FTL:   visit  TableColGroupFlow {}", "1");

 }
 fn visit_2(&mut self) {
  debug!("FTL:   visit  TableColGroupFlow {}", "2");

 }
 fn visit_3(&mut self) {
  debug!("FTL:   visit  TableColGroupFlow {}", "3");

 }
}
impl FtlNode for BlockFlow {
 fn with_all_children(&mut self, func: |&mut FtlNode|) {
  for child in self.base.children.mut_iter() {
    func(as_ftl_node(child));
  }
 }
 fn visit_0(&mut self) {
  debug!("FTL:   visit  BlockFlow {}", "0");
  self.ftl_attrs.bb = (blockflow_bb(self.fragment.style().get_border().border_bottom_width));
  debug!("FTL:     blockflow_bb {}", self.ftl_attrs.bb);
  self.ftl_attrs.bl = (blockflow_bl(self.fragment.style().get_border().border_left_width));
  debug!("FTL:     blockflow_bl {}", self.ftl_attrs.bl);
  self.ftl_attrs.br = (blockflow_br(self.fragment.style().get_border().border_right_width));
  debug!("FTL:     blockflow_br {}", self.ftl_attrs.br);
  self.ftl_attrs.bt = (blockflow_bt(self.fragment.style().get_border().border_top_width));
  debug!("FTL:     blockflow_bt {}", self.ftl_attrs.bt);
  self.ftl_attrs.mb = (blockflow_mb(self.fragment.style().get_margin().margin_bottom, self.base.ftl_attrs.availablewidth));
  debug!("FTL:     blockflow_mb {}", self.ftl_attrs.mb);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.mt = (blockflow_mt(self.base.ftl_attrs.availablewidth, self.fragment.style().get_margin().margin_top));
  debug!("FTL:     blockflow_mt {}", self.ftl_attrs.mt);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.pb = (blockflow_pb(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_bottom));
  debug!("FTL:     blockflow_pb {}", self.ftl_attrs.pb);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.pl = (blockflow_pl(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_left));
  debug!("FTL:     blockflow_pl {}", self.ftl_attrs.pl);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.pr = (blockflow_pr(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_right));
  debug!("FTL:     blockflow_pr {}", self.ftl_attrs.pr);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.pt = (blockflow_pt(self.base.ftl_attrs.availablewidth, self.fragment.style().get_padding().padding_top));
  debug!("FTL:     blockflow_pt {}", self.ftl_attrs.pt);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.selfintrinsheight = (blockflow_selfIntrinsHeight(self.fragment.style().get_box().height));
  debug!("FTL:     blockflow_selfIntrinsHeight {}", self.ftl_attrs.selfintrinsheight);
  self.ftl_attrs.selfintrinswidth = (blockflow_selfIntrinsWidth(self.base.ftl_attrs.availablewidth, self.fragment.style().get_box().width));
  debug!("FTL:     blockflow_selfIntrinsWidth {}", self.ftl_attrs.selfintrinswidth);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  self.ftl_attrs.mbpvert = (blockflow_mbpVert(self.ftl_attrs.pb, self.ftl_attrs.mt, self.ftl_attrs.mb, self.ftl_attrs.pt, self.ftl_attrs.bb, self.ftl_attrs.bt));
  debug!("FTL:     blockflow_mbpVert {}", self.ftl_attrs.mbpvert);
  debug!("FTL:         pb {}", self.ftl_attrs.pb);
  debug!("FTL:         mt {}", self.ftl_attrs.mt);
  debug!("FTL:         mb {}", self.ftl_attrs.mb);
  debug!("FTL:         pt {}", self.ftl_attrs.pt);
  debug!("FTL:         bb {}", self.ftl_attrs.bb);
  debug!("FTL:         bt {}", self.ftl_attrs.bt);
  self.ftl_attrs.ml = (blockflow_ml(self.base.ftl_attrs.availablewidth, self.fragment.style().get_box().width, self.fragment.style().get_margin().margin_left, self.ftl_attrs.bl, self.fragment.style().get_margin().margin_right, self.ftl_attrs.pl, self.ftl_attrs.selfintrinswidth, self.ftl_attrs.br, self.ftl_attrs.pr));
  debug!("FTL:     blockflow_ml {}", self.ftl_attrs.ml);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  debug!("FTL:         bl {}", self.ftl_attrs.bl);
  debug!("FTL:         pl {}", self.ftl_attrs.pl);
  debug!("FTL:         selfIntrinsWidth {}", self.ftl_attrs.selfintrinswidth);
  debug!("FTL:         br {}", self.ftl_attrs.br);
  debug!("FTL:         pr {}", self.ftl_attrs.pr);
  self.ftl_attrs.mr = (blockflow_mr(self.base.ftl_attrs.availablewidth, self.fragment.style().get_margin().margin_left, self.fragment.style().get_box().width, self.ftl_attrs.bl, self.fragment.style().get_margin().margin_right, self.ftl_attrs.pl, self.ftl_attrs.selfintrinswidth, self.ftl_attrs.br, self.ftl_attrs.pr));
  debug!("FTL:     blockflow_mr {}", self.ftl_attrs.mr);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  debug!("FTL:         bl {}", self.ftl_attrs.bl);
  debug!("FTL:         pl {}", self.ftl_attrs.pl);
  debug!("FTL:         selfIntrinsWidth {}", self.ftl_attrs.selfintrinswidth);
  debug!("FTL:         br {}", self.ftl_attrs.br);
  debug!("FTL:         pr {}", self.ftl_attrs.pr);
  self.ftl_attrs.mbphoriz = (blockflow_mbpHoriz(self.ftl_attrs.mr, self.ftl_attrs.bl, self.ftl_attrs.pl, self.ftl_attrs.br, self.ftl_attrs.ml, self.ftl_attrs.pr));
  debug!("FTL:     blockflow_mbpHoriz {}", self.ftl_attrs.mbphoriz);
  debug!("FTL:         mr {}", self.ftl_attrs.mr);
  debug!("FTL:         bl {}", self.ftl_attrs.bl);
  debug!("FTL:         pl {}", self.ftl_attrs.pl);
  debug!("FTL:         br {}", self.ftl_attrs.br);
  debug!("FTL:         ml {}", self.ftl_attrs.ml);
  debug!("FTL:         pr {}", self.ftl_attrs.pr);
  self.ftl_attrs.computedwidth = (blockflow_computedWidth(self.is_root, self.base.ftl_attrs.availablewidth, self.fragment.style().get_box().width, self.ftl_attrs.mbphoriz, self.ftl_attrs.selfintrinswidth, self.screenwidth));
  debug!("FTL:     blockflow_computedWidth {}", self.ftl_attrs.computedwidth);
  debug!("FTL:         is_root {}", self.is_root);
  debug!("FTL:         availableWidth {}", self.base.ftl_attrs.availablewidth);
  debug!("FTL:         mbpHoriz {}", self.ftl_attrs.mbphoriz);
  debug!("FTL:         selfIntrinsWidth {}", self.ftl_attrs.selfintrinswidth);
  debug!("FTL:         screenwidth {}", self.screenwidth);
  self.base.position.size.width = (blockflow_flowWidth(self.is_root, self.ftl_attrs.bl, self.ftl_attrs.pl, self.ftl_attrs.computedwidth, self.ftl_attrs.br, self.screenwidth, self.ftl_attrs.pr));
  debug!("FTL:     blockflow_flowWidth {}", self.base.position.size.width);
  debug!("FTL:         is_root {}", self.is_root);
  debug!("FTL:         bl {}", self.ftl_attrs.bl);
  debug!("FTL:         pl {}", self.ftl_attrs.pl);
  debug!("FTL:         computedWidth {}", self.ftl_attrs.computedwidth);
  debug!("FTL:         br {}", self.ftl_attrs.br);
  debug!("FTL:         screenwidth {}", self.screenwidth);
  debug!("FTL:         pr {}", self.ftl_attrs.pr);
  self.base.ftl_attrs.totalwidth = (blockflow_totalWidth(self.base.position.size.width, self.ftl_attrs.mr, self.ftl_attrs.ml));
  debug!("FTL:     blockflow_totalWidth {}", self.base.ftl_attrs.totalwidth);
  debug!("FTL:         flowWidth {}", self.base.position.size.width);
  debug!("FTL:         mr {}", self.ftl_attrs.mr);
  debug!("FTL:         ml {}", self.ftl_attrs.ml);

  let flowchildren_availablewidth_init = ((Au(0)));
  let mut flowchildren_availablewidth_last = (flowchildren_availablewidth_init);
  debug!("FTL:       init flowChildren@availableWidth {}", flowchildren_availablewidth_init);
  debug!("FTL:     last init flowchildren_availablewidth_last {}", flowchildren_availablewidth_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.availablewidth = ((self.ftl_attrs.computedwidth ));
      flowchildren_availablewidth_last = child.ftl_attrs.availablewidth;
      debug!("FTL:          step flowChildren@availableWidth {}", child.ftl_attrs.availablewidth);
    first = false;
    old_child = Some(child);
  }

  }


 }
 fn visit_1(&mut self) {
  debug!("FTL:   visit  BlockFlow {}", "1");

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
  debug!("FTL:       init childsHeight {}", childsheight_init);
  debug!("FTL:     last init childsheight {}", self.ftl_attrs.childsheight);
    let childswidth_init = ((Au(0)));
  self.ftl_attrs.childswidth = (childswidth_init);
  debug!("FTL:       init childsWidth {}", childswidth_init);
  debug!("FTL:     last init childswidth {}", self.ftl_attrs.childswidth);
    let flowchildren_bottom_init = ((self.ftl_attrs.pt + self.ftl_attrs.bt ));
  let mut flowchildren_bottom_last = (flowchildren_bottom_init);
  debug!("FTL:       init flowChildren@bottom {}", flowchildren_bottom_init);
  debug!("FTL:     last init flowchildren_bottom_last {}", flowchildren_bottom_last);
    let flowchildren_right_init = ((Au(0)));
  let mut flowchildren_right_last = (flowchildren_right_init);
  debug!("FTL:       init flowChildren@right {}", flowchildren_right_init);
  debug!("FTL:     last init flowchildren_right_last {}", flowchildren_right_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      self.ftl_attrs.childsheight = (((self.ftl_attrs.childsheight + child.ftl_attrs.totalheight)));
      debug!("FTL:          step childsHeight {}", self.ftl_attrs.childsheight);
      self.ftl_attrs.childswidth = ((max(self.ftl_attrs.childswidth, child.ftl_attrs.totalwidth)));
      debug!("FTL:          step childsWidth {}", self.ftl_attrs.childswidth);
      child.ftl_attrs.bottom = ((((if first { flowchildren_bottom_init } else { old_child.get_ref().ftl_attrs.bottom }) + child.ftl_attrs.totalheight)));
      flowchildren_bottom_last = child.ftl_attrs.bottom;
      debug!("FTL:          step flowChildren@bottom {}", child.ftl_attrs.bottom);
      child.ftl_attrs.right = (((child.ftl_attrs.totalwidth + self.ftl_attrs.pl + self.ftl_attrs.bl)));
      flowchildren_right_last = child.ftl_attrs.right;
      debug!("FTL:          step flowChildren@right {}", child.ftl_attrs.right);
    first = false;
    old_child = Some(child);
  }

  }


  let flowchildren_containingx_init = ((Au(0)));
  let mut flowchildren_containingx_last = (flowchildren_containingx_init);
  debug!("FTL:       init flowChildren@containingX {}", flowchildren_containingx_init);
  debug!("FTL:     last init flowchildren_containingx_last {}", flowchildren_containingx_last);
    let flowchildren_containingy_init = ((Au(0)));
  let mut flowchildren_containingy_last = (flowchildren_containingy_init);
  debug!("FTL:       init flowChildren@containingY {}", flowchildren_containingy_init);
  debug!("FTL:     last init flowchildren_containingy_last {}", flowchildren_containingy_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.containingx = ((child.ftl_attrs.right - child.ftl_attrs.totalwidth ));
      flowchildren_containingx_last = child.ftl_attrs.containingx;
      debug!("FTL:          step flowChildren@containingX {}", child.ftl_attrs.containingx);
      child.ftl_attrs.containingy = ((child.ftl_attrs.bottom - child.ftl_attrs.totalheight ));
      flowchildren_containingy_last = child.ftl_attrs.containingy;
      debug!("FTL:          step flowChildren@containingY {}", child.ftl_attrs.containingy);
    first = false;
    old_child = Some(child);
  }

  }

  self.base.position.size.height = (blockflow_flowHeight(self.ftl_attrs.pb, self.ftl_attrs.childsheight, self.ftl_attrs.pt, self.ftl_attrs.selfintrinsheight, self.ftl_attrs.bb, self.ftl_attrs.bt));
  debug!("FTL:     blockflow_flowHeight {}", self.base.position.size.height);
  debug!("FTL:         pb {}", self.ftl_attrs.pb);
  debug!("FTL:         childsHeight {}", self.ftl_attrs.childsheight);
  debug!("FTL:         pt {}", self.ftl_attrs.pt);
  debug!("FTL:         selfIntrinsHeight {}", self.ftl_attrs.selfintrinsheight);
  debug!("FTL:         bb {}", self.ftl_attrs.bb);
  debug!("FTL:         bt {}", self.ftl_attrs.bt);
  self.base.ftl_attrs.totalheight = (blockflow_totalHeight(self.base.position.size.height, self.ftl_attrs.mt, self.ftl_attrs.mb));
  debug!("FTL:     blockflow_totalHeight {}", self.base.ftl_attrs.totalheight);
  debug!("FTL:         flowHeight {}", self.base.position.size.height);
  debug!("FTL:         mt {}", self.ftl_attrs.mt);
  debug!("FTL:         mb {}", self.ftl_attrs.mb);

 }
 fn visit_2(&mut self) {
  debug!("FTL:   visit  BlockFlow {}", "2");
  self.base.display_list = (blockflow_display_list(self.base.position.size.height, self.base.position.size.width, self.ftl_attrs.bl, self.ftl_attrs.mt, self.base.ftl_attrs.absy, self.base.ftl_attrs.absx, self.ftl_attrs.bb, self.ftl_attrs.br, self.ftl_attrs.ml, &mut self.fragment, self.ftl_attrs.bt));
  debug!("FTL:         flowHeight {}", self.base.position.size.height);
  debug!("FTL:         flowWidth {}", self.base.position.size.width);
  debug!("FTL:         bl {}", self.ftl_attrs.bl);
  debug!("FTL:         mt {}", self.ftl_attrs.mt);
  debug!("FTL:         absY {}", self.base.ftl_attrs.absy);
  debug!("FTL:         absX {}", self.base.ftl_attrs.absx);
  debug!("FTL:         bb {}", self.ftl_attrs.bb);
  debug!("FTL:         br {}", self.ftl_attrs.br);
  debug!("FTL:         ml {}", self.ftl_attrs.ml);
  debug!("FTL:         bt {}", self.ftl_attrs.bt);
  self.base.position.origin.x = (blockflow_flowX(self.base.ftl_attrs.containingx, self.ftl_attrs.ml));
  debug!("FTL:     blockflow_flowX {}", self.base.position.origin.x);
  debug!("FTL:         containingX {}", self.base.ftl_attrs.containingx);
  debug!("FTL:         ml {}", self.ftl_attrs.ml);
  self.base.position.origin.y = (blockflow_flowY(self.base.ftl_attrs.containingy, self.ftl_attrs.mt));
  debug!("FTL:     blockflow_flowY {}", self.base.position.origin.y);
  debug!("FTL:         containingY {}", self.base.ftl_attrs.containingy);
  debug!("FTL:         mt {}", self.ftl_attrs.mt);

  let flowchildren_absx_init = ((Au(0)));
  let mut flowchildren_absx_last = (flowchildren_absx_init);
  debug!("FTL:       init flowChildren@absX {}", flowchildren_absx_init);
  debug!("FTL:     last init flowchildren_absx_last {}", flowchildren_absx_last);
    let flowchildren_absy_init = ((Au(0)));
  let mut flowchildren_absy_last = (flowchildren_absy_init);
  debug!("FTL:       init flowChildren@absY {}", flowchildren_absy_init);
  debug!("FTL:     last init flowchildren_absy_last {}", flowchildren_absy_last);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      child.ftl_attrs.absx = ((child.ftl_attrs.containingx + self.base.ftl_attrs.absx + self.ftl_attrs.ml ));
      flowchildren_absx_last = child.ftl_attrs.absx;
      debug!("FTL:          step flowChildren@absX {}", child.ftl_attrs.absx);
      child.ftl_attrs.absy = ((child.ftl_attrs.containingy + self.base.ftl_attrs.absy + self.ftl_attrs.mt ));
      flowchildren_absy_last = child.ftl_attrs.absy;
      debug!("FTL:          step flowChildren@absY {}", child.ftl_attrs.absy);
    first = false;
    old_child = Some(child);
  }

  }


 }
 fn visit_3(&mut self) {
  debug!("FTL:   visit  BlockFlow {}", "3");

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
  debug!("FTL:       init makeLists {}", makelists_init);
  debug!("FTL:     last init makelists {}", self.base.ftl_attrs.makelists);
    { // Appease the borrow checker
  let mut old_child: Option<&mut BaseFlow> = None;
  let mut children = self.base.children.mut_iter().map(|x| mut_base(x));
  let mut first = true;
  loop {
    let child = match children.next() { None => {break;} Some(c) => {c} };
      self.base.ftl_attrs.makelists = ((merge_lists(&mut self.base.display_list, &mut child.display_list)));
      debug!("FTL:          step makeLists {}", self.base.ftl_attrs.makelists);
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
