/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Creates flows and fragments from a DOM tree via a bottom-up, incremental traversal of the DOM.
//!
//! Each step of the traversal considers the node and existing flow, if there is one. If a node is
//! not dirty and an existing flow exists, then the traversal reuses that flow. Otherwise, it
//! proceeds to construct either a flow or a `ConstructionItem`. A construction item is a piece of
//! intermediate data that goes with a DOM node and hasn't found its "home" yet-maybe it's a box,
//! maybe it's an absolute or fixed position thing that hasn't found its containing block yet.
//! Construction items bubble up the tree from children to parents until they find their homes.
//!
//! TODO(pcwalton): There is no incremental reflow yet. This scheme requires that nodes either have
//! weak references to flows or that there be some mechanism to efficiently (O(1) time) "blow
//! apart" a flow tree and have the flows migrate "home" to their respective DOM nodes while we
//! perform flow tree construction. The precise mechanism for this will take some experimentation
//! to get right.
//!
//! TODO(pcwalton): This scheme should be amenable to parallelization, but, of course, that's not
//! yet implemented.

use css::node_style::StyledNode;
use layout::block::BlockFlow;
use layout::context::LayoutContext;
use layout::floats::FloatKind;
use layout::flow::{Flow, ImmutableFlowUtils, MutableOwnedFlowUtils};
use layout::flow::{Descendants, AbsDescendants};
use layout::flow_list::{Rawlink};
use layout::fragment::{Fragment, GenericFragment, IframeFragment, IframeFragmentInfo, ImageFragment, ImageFragmentInfo};
use layout::fragment::{SpecificFragmentInfo, TableFragment, TableCellFragment, TableColumnFragment, TableColumnFragmentInfo};
use layout::fragment::{TableRowFragment, TableWrapperFragment, UnscannedTextFragment, UnscannedTextFragmentInfo};
use layout::inline::{FragmentIndex, InlineFragments, InlineFlow};
use layout::table_wrapper::TableWrapperFlow;
use layout::table::TableFlow;
use layout::table_caption::TableCaptionFlow;
use layout::table_colgroup::TableColGroupFlow;
use layout::table_rowgroup::TableRowGroupFlow;
use layout::table_row::TableRowFlow;
use layout::table_cell::TableCellFlow;
use layout::text::TextRunScanner;
use layout::util::{LayoutDataAccess, OpaqueNodeMethods};
use layout::wrapper::{PostorderNodeMutTraversal, TLayoutNode, ThreadSafeLayoutNode};
use layout::wrapper::{Before, BeforeBlock, After, AfterBlock, Normal};

use gfx::display_list::OpaqueNode;
use gfx::font_context::FontContext;
use script::dom::bindings::js::JS;
use script::dom::element::{HTMLIFrameElementTypeId, HTMLImageElementTypeId};
use script::dom::element::{HTMLObjectElementTypeId};
use script::dom::element::{HTMLTableColElementTypeId, HTMLTableDataCellElementTypeId};
use script::dom::element::{HTMLTableElementTypeId, HTMLTableHeaderCellElementTypeId};
use script::dom::element::{HTMLTableRowElementTypeId, HTMLTableSectionElementTypeId};
use script::dom::node::{CommentNodeTypeId, DoctypeNodeTypeId, DocumentFragmentNodeTypeId};
use script::dom::node::{DocumentNodeTypeId, ElementNodeTypeId, ProcessingInstructionNodeTypeId};
use script::dom::node::{TextNodeTypeId};
use script::dom::text::Text;
use servo_util::namespace;
use servo_util::range::Range;
use servo_util::str::is_whitespace;
use servo_util::url::{is_image_data, parse_url};
use std::mem;
use style::ComputedValues;
use style::computed_values::{display, position, float, white_space};
use sync::Arc;
use url::Url;

/// The results of flow construction for a DOM node.
pub enum ConstructionResult {
    /// This node contributes nothing at all (`display: none`). Alternately, this is what newly
    /// created nodes have their `ConstructionResult` set to.
    NoConstructionResult,

    /// This node contributed a flow at the proper position in the tree.
    /// Nothing more needs to be done for this node. It has bubbled up fixed
    /// and absolute descendant flows that have a CB above it.
    FlowConstructionResult(Box<Flow:Share>, AbsDescendants),

    /// This node contributed some object or objects that will be needed to construct a proper flow
    /// later up the tree, but these objects have not yet found their home.
    ConstructionItemConstructionResult(ConstructionItem),
}

impl ConstructionResult {
    fn destroy(&mut self) {
        match *self {
            NoConstructionResult => {}
            FlowConstructionResult(ref mut flow, _) => flow.destroy(),
            ConstructionItemConstructionResult(ref mut item) => item.destroy(),
        }
    }
}

/// Represents the output of flow construction for a DOM node that has not yet resulted in a
/// complete flow. Construction items bubble up the tree until they find a `Flow` to be
/// attached to.
pub enum ConstructionItem {
    /// Inline fragments and associated {ib} splits that have not yet found flows.
    InlineFragmentsConstructionItem(InlineFragmentsConstructionResult),
    /// Potentially ignorable whitespace.
    WhitespaceConstructionItem(OpaqueNode, Arc<ComputedValues>),
    /// TableColumn Fragment
    TableColumnFragmentConstructionItem(Fragment),
}

impl ConstructionItem {
    fn destroy(&mut self) {
        match *self {
            InlineFragmentsConstructionItem(ref mut result) => {
                for split in result.splits.mut_iter() {
                    split.destroy()
                }
            }
            WhitespaceConstructionItem(..) => {}
            TableColumnFragmentConstructionItem(_) => {}
        }
    }
}

/// Represents inline fragments and {ib} splits that are bubbling up from an inline.
pub struct InlineFragmentsConstructionResult {
    /// Any {ib} splits that we're bubbling up.
    pub splits: Vec<InlineBlockSplit>,

    /// Any fragments that succeed the {ib} splits.
    pub fragments: InlineFragments,

    /// Any absolute descendants that we're bubbling up.
    pub abs_descendants: AbsDescendants,
}

/// Represents an {ib} split that has not yet found the containing block that it belongs to. This
/// is somewhat tricky. An example may be helpful. For this DOM fragment:
///
///     <span>
///     A
///     <div>B</div>
///     C
///     </span>
///
/// The resulting `ConstructionItem` for the outer `span` will be:
///
///     InlineFragmentsConstructionItem(Some(~[
///         InlineBlockSplit {
///             predecessor_fragments: ~[
///                 A
///             ],
///             block: ~BlockFlow {
///                 B
///             },
///         }),~[
///             C
///         ])
pub struct InlineBlockSplit {
    /// The inline fragments that precede the flow.
    pub predecessors: InlineFragments,

    /// The flow that caused this {ib} split.
    pub flow: Box<Flow:Share>,
}

impl InlineBlockSplit {
    fn destroy(&mut self) {
        self.flow.destroy()
    }
}

/// Holds inline fragments that we're gathering for children of an inline node.
struct InlineFragmentsAccumulator {
    /// The list of fragments.
    fragments: InlineFragments,

    /// Whether we've created a range to enclose all the fragments. This will be true if the outer node
    /// is an inline and false otherwise.
    has_enclosing_range: bool,
}

impl InlineFragmentsAccumulator {
    fn new() -> InlineFragmentsAccumulator {
        InlineFragmentsAccumulator {
            fragments: InlineFragments::new(),
            has_enclosing_range: false,
        }
    }

    fn from_inline_node(node: &ThreadSafeLayoutNode) -> InlineFragmentsAccumulator {
        let mut fragments = InlineFragments::new();
        fragments.map.push(node.style().clone(), Range::empty());
        InlineFragmentsAccumulator {
            fragments: fragments,
            has_enclosing_range: true,
        }
    }

    fn finish(self) -> InlineFragments {
        let InlineFragmentsAccumulator {
            fragments: mut fragments,
            has_enclosing_range
        } = self;

        if has_enclosing_range {
            let len = FragmentIndex(fragments.len() as int);
            fragments.map.get_mut(FragmentIndex(0)).range.extend_to(len);
        }
        fragments
    }
}

enum WhitespaceStrippingMode {
    NoWhitespaceStripping,
    StripWhitespaceFromStart,
    StripWhitespaceFromEnd,
}

/// An object that knows how to create flows.
pub struct FlowConstructor<'a> {
    /// The layout context.
    pub layout_context: &'a mut LayoutContext,

    /// An optional font context. If this is `None`, then we fetch the font context from the
    /// layout context.
    ///
    /// FIXME(pcwalton): This is pretty bogus and is basically just a workaround for libgreen
    /// having slow TLS.
    pub font_context: Option<Box<FontContext>>,
}

impl<'a> FlowConstructor<'a> {
    /// Creates a new flow constructor.
    pub fn new(layout_context: &'a mut LayoutContext, font_context: Option<Box<FontContext>>)
               -> FlowConstructor<'a> {
        FlowConstructor {
            layout_context: layout_context,
            font_context: font_context,
        }
    }

    fn font_context<'a>(&'a mut self) -> &'a mut FontContext {
        match self.font_context {
            Some(ref mut font_context) => {
                let font_context: &mut FontContext = *font_context;
                font_context
            }
            None => self.layout_context.font_context(),
        }
    }

    /// Destroys this flow constructor and retrieves the font context.
    pub fn unwrap_font_context(self) -> Option<Box<FontContext>> {
        let FlowConstructor {
            font_context,
            ..
        } = self;
        font_context
    }

    /// Builds the `ImageFragmentInfo` for the given image. This is out of line to guide inlining.
    fn build_fragment_info_for_image(&mut self, node: &ThreadSafeLayoutNode, url: Option<Url>)
                                -> SpecificFragmentInfo {
        match url {
            None => GenericFragment,
            Some(url) => {
                // FIXME(pcwalton): The fact that image fragments store the cache within them makes
                // little sense to me.
                ImageFragment(ImageFragmentInfo::new(node, url, self.layout_context.image_cache.clone()))
            }
        }
    }

    /// Builds specific `Fragment` info for the given node.
    pub fn build_specific_fragment_info_for_node(&mut self, node: &ThreadSafeLayoutNode)
                                            -> SpecificFragmentInfo {
        match node.type_id() {
            Some(ElementNodeTypeId(HTMLImageElementTypeId)) => {
                self.build_fragment_info_for_image(node, node.image_url())
            }
            Some(ElementNodeTypeId(HTMLIFrameElementTypeId)) => {
                IframeFragment(IframeFragmentInfo::new(node))
            }
            Some(ElementNodeTypeId(HTMLObjectElementTypeId)) => {
                let data = node.get_object_data(&self.layout_context.url);
                self.build_fragment_info_for_image(node, data)
            }
            Some(ElementNodeTypeId(HTMLTableElementTypeId)) => TableWrapperFragment,
            Some(ElementNodeTypeId(HTMLTableColElementTypeId)) => {
                TableColumnFragment(TableColumnFragmentInfo::new(node))
            }
            Some(ElementNodeTypeId(HTMLTableDataCellElementTypeId)) |
            Some(ElementNodeTypeId(HTMLTableHeaderCellElementTypeId)) => TableCellFragment,
            Some(ElementNodeTypeId(HTMLTableRowElementTypeId)) |
            Some(ElementNodeTypeId(HTMLTableSectionElementTypeId)) => TableRowFragment,
            None | Some(TextNodeTypeId) => UnscannedTextFragment(UnscannedTextFragmentInfo::new(node)),
            _ => GenericFragment,
        }
    }

    /// Creates an inline flow from a set of inline fragments, then adds it as a child of the given flow
    /// or pushes it onto the given flow list.
    ///
    /// `#[inline(always)]` because this is performance critical and LLVM will not inline it
    /// otherwise.
    #[inline(always)]
    fn flush_inline_fragments_to_flow_or_list(&mut self,
                                          fragment_accumulator: InlineFragmentsAccumulator,
                                          flow: &mut Box<Flow:Share>,
                                          flow_list: &mut Vec<Box<Flow:Share>>,
                                          whitespace_stripping: WhitespaceStrippingMode,
                                          node: &ThreadSafeLayoutNode) {
        let mut fragments = fragment_accumulator.finish();
        if fragments.len() == 0 {
            return
        }

        match whitespace_stripping {
            NoWhitespaceStripping => {}
            StripWhitespaceFromStart => {
                strip_ignorable_whitespace_from_start(&mut fragments);
                if fragments.len() == 0 {
                    return
                }
            }
            StripWhitespaceFromEnd => {
                strip_ignorable_whitespace_from_end(&mut fragments);
                if fragments.len() == 0 {
                    return
                }
            }
        }

        let mut inline_flow = box InlineFlow::from_fragments((*node).clone(), fragments);
        inline_flow.compute_minimum_ascent_and_descent(self.font_context(), &**node.style());
        let mut inline_flow = inline_flow as Box<Flow:Share>;
        TextRunScanner::new().scan_for_runs(self.font_context(), inline_flow);
        inline_flow.finish(self.layout_context);

        if flow.need_anonymous_flow(inline_flow) {
            flow_list.push(inline_flow)
        } else {
            flow.add_new_child(inline_flow)
        }
    }

    fn build_block_flow_using_children_construction_result(&mut self,
                                                           flow: &mut Box<Flow:Share>,
                                                           consecutive_siblings:
                                                           &mut Vec<Box<Flow:Share>>,
                                                           node: &ThreadSafeLayoutNode,
                                                           kid: ThreadSafeLayoutNode,
                                                           inline_fragment_accumulator:
                                                           &mut InlineFragmentsAccumulator,
                                                           abs_descendants: &mut Descendants,
                                                           first_fragment: &mut bool) {
        match kid.swap_out_construction_result() {
            NoConstructionResult => {}
            FlowConstructionResult(kid_flow, kid_abs_descendants) => {
                // If kid_flow is TableCaptionFlow, kid_flow should be added under
                // TableWrapperFlow.
                if flow.is_table() && kid_flow.is_table_caption() {
                    kid.set_flow_construction_result(FlowConstructionResult(
                            kid_flow,
                            Descendants::new()))
                } else if flow.need_anonymous_flow(kid_flow) {
                    consecutive_siblings.push(kid_flow)
                } else {
                    // Strip ignorable whitespace from the start of this flow per CSS 2.1 §
                    // 9.2.1.1.
                    let whitespace_stripping = if flow.is_table_kind() || *first_fragment {
                        *first_fragment = false;
                        StripWhitespaceFromStart
                    } else {
                        NoWhitespaceStripping
                    };

                    // Flush any inline fragments that we were gathering up. This allows us to handle
                    // {ib} splits.
                    debug!("flushing {} inline box(es) to flow A",
                           inline_fragment_accumulator.fragments.len());
                    self.flush_inline_fragments_to_flow_or_list(
                        mem::replace(inline_fragment_accumulator, InlineFragmentsAccumulator::new()),
                        flow,
                        consecutive_siblings,
                        whitespace_stripping,
                        node);
                    if !consecutive_siblings.is_empty() {
                        let consecutive_siblings = mem::replace(consecutive_siblings, vec!());
                        self.generate_anonymous_missing_child(consecutive_siblings,
                                                              flow,
                                                              node);
                    }
                    flow.add_new_child(kid_flow);
                }
                abs_descendants.push_descendants(kid_abs_descendants);
            }
            ConstructionItemConstructionResult(InlineFragmentsConstructionItem(
                    InlineFragmentsConstructionResult {
                        splits: splits,
                        fragments: successor_fragments,
                        abs_descendants: kid_abs_descendants,
                    })) => {
                // Add any {ib} splits.
                for split in splits.move_iter() {
                    // Pull apart the {ib} split object and push its predecessor fragments
                    // onto the list.
                    let InlineBlockSplit {
                        predecessors: predecessors,
                        flow: kid_flow
                    } = split;
                    inline_fragment_accumulator.fragments.push_all(predecessors);

                    // If this is the first fragment in flow, then strip ignorable
                    // whitespace per CSS 2.1 § 9.2.1.1.
                    let whitespace_stripping = if *first_fragment {
                        *first_fragment = false;
                        StripWhitespaceFromStart
                    } else {
                        NoWhitespaceStripping
                    };

                    // Flush any inline fragments that we were gathering up.
                    debug!("flushing {} inline box(es) to flow A",
                           inline_fragment_accumulator.fragments.len());
                    self.flush_inline_fragments_to_flow_or_list(
                            mem::replace(inline_fragment_accumulator,
                                         InlineFragmentsAccumulator::new()),
                            flow,
                            consecutive_siblings,
                            whitespace_stripping,
                            node);

                    // Push the flow generated by the {ib} split onto our list of
                    // flows.
                    if flow.need_anonymous_flow(kid_flow) {
                        consecutive_siblings.push(kid_flow)
                    } else {
                        flow.add_new_child(kid_flow)
                    }
                }

                // Add the fragments to the list we're maintaining.
                inline_fragment_accumulator.fragments.push_all(successor_fragments);
                abs_descendants.push_descendants(kid_abs_descendants);
            }
            ConstructionItemConstructionResult(WhitespaceConstructionItem(..)) => {
                // Nothing to do here.
            }
            ConstructionItemConstructionResult(TableColumnFragmentConstructionItem(_)) => {
                // TODO: Implement anonymous table objects for missing parents
                // CSS 2.1 § 17.2.1, step 3-2
            }
        }
    }

    /// Build block flow for current node using information from children nodes.
    ///
    /// Consume results from children and combine them, handling {ib} splits.
    /// Block flows and inline flows thus created will become the children of
    /// this block flow.
    /// Also, deal with the absolute and fixed descendants bubbled up by
    /// children nodes.
    fn build_flow_using_children(&mut self,
                                 mut flow: Box<Flow:Share>,
                                 node: &ThreadSafeLayoutNode)
                                 -> ConstructionResult {
        // Gather up fragments for the inline flows we might need to create.
        let mut inline_fragment_accumulator = InlineFragmentsAccumulator::new();
        let mut consecutive_siblings = vec!();
        let mut first_fragment = true;

        // List of absolute descendants, in tree order.
        let mut abs_descendants = Descendants::new();
        for kid in node.children() {
            if kid.get_pseudo_element_type() != Normal {
                self.process(&kid);
            }

            self.build_block_flow_using_children_construction_result(&mut flow,
                                                                     &mut consecutive_siblings,
                                                                     node,
                                                                     kid,
                                                                     &mut inline_fragment_accumulator,
                                                                     &mut abs_descendants,
                                                                     &mut first_fragment);
        }

        // Perform a final flush of any inline fragments that we were gathering up to handle {ib}
        // splits, after stripping ignorable whitespace.
        self.flush_inline_fragments_to_flow_or_list(inline_fragment_accumulator,
                                                &mut flow,
                                                &mut consecutive_siblings,
                                                StripWhitespaceFromEnd,
                                                node);
        if !consecutive_siblings.is_empty() {
            self.generate_anonymous_missing_child(consecutive_siblings, &mut flow, node);
        }

        // The flow is done.
        flow.finish(self.layout_context);
        let is_positioned = flow.as_block().is_positioned();
        let is_fixed_positioned = flow.as_block().is_fixed();
        let is_absolutely_positioned = flow.as_block().is_absolutely_positioned();
        if is_positioned {
            // This is the CB for all the absolute descendants.
            flow.set_abs_descendants(abs_descendants);
            abs_descendants = Descendants::new();

            if is_fixed_positioned || is_absolutely_positioned {
                // This is now the only absolute flow in the subtree which hasn't yet
                // reached its CB.
                abs_descendants.push(Rawlink::some(flow));
            }
        }
        FlowConstructionResult(flow, abs_descendants)
    }

    /// Builds a flow for a node with `display: block`. This yields a `BlockFlow` with possibly
    /// other `BlockFlow`s or `InlineFlow`s underneath it, depending on whether {ib} splits needed
    /// to happen.
    fn build_flow_for_block(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let flow = box BlockFlow::from_node(self, node) as Box<Flow:Share>;
        self.build_flow_using_children(flow, node)
    }

    /// Builds the flow for a node with `float: {left|right}`. This yields a float `BlockFlow` with
    /// a `BlockFlow` underneath it.
    fn build_flow_for_floated_block(&mut self, node: &ThreadSafeLayoutNode, float_kind: FloatKind)
                                    -> ConstructionResult {
        let flow = box BlockFlow::float_from_node(self, node, float_kind) as Box<Flow:Share>;
        self.build_flow_using_children(flow, node)
    }

    /// Concatenates the fragments of kids, adding in our own borders/padding/margins if necessary.
    /// Returns the `InlineFragmentsConstructionResult`, if any. There will be no
    /// `InlineFragmentsConstructionResult` if this node consisted entirely of ignorable whitespace.
    fn build_fragments_for_nonreplaced_inline_content(&mut self, node: &ThreadSafeLayoutNode)
                                                  -> ConstructionResult {
        let mut opt_inline_block_splits: Vec<InlineBlockSplit> = Vec::new();
        let mut fragment_accumulator = InlineFragmentsAccumulator::from_inline_node(node);
        let mut abs_descendants = Descendants::new();

        // Concatenate all the fragments of our kids, creating {ib} splits as necessary.
        for kid in node.children() {
            if kid.get_pseudo_element_type() != Normal {
                self.process(&kid);
            }
            match kid.swap_out_construction_result() {
                NoConstructionResult => {}
                FlowConstructionResult(flow, kid_abs_descendants) => {
                    // {ib} split. Flush the accumulator to our new split and make a new
                    // accumulator to hold any subsequent fragments we come across.
                    let split = InlineBlockSplit {
                        predecessors:
                            mem::replace(&mut fragment_accumulator,
                                         InlineFragmentsAccumulator::from_inline_node(node)).finish(),
                        flow: flow,
                    };
                    opt_inline_block_splits.push(split);
                    abs_descendants.push_descendants(kid_abs_descendants);
                }
                ConstructionItemConstructionResult(InlineFragmentsConstructionItem(
                        InlineFragmentsConstructionResult {
                            splits: splits,
                            fragments: successors,
                            abs_descendants: kid_abs_descendants,
                        })) => {

                    // Bubble up {ib} splits.
                    for split in splits.move_iter() {
                        let InlineBlockSplit {
                            predecessors: predecessors,
                            flow: kid_flow
                        } = split;
                        fragment_accumulator.fragments.push_all(predecessors);

                        let split = InlineBlockSplit {
                            predecessors:
                                mem::replace(&mut fragment_accumulator,
                                             InlineFragmentsAccumulator::from_inline_node(node))
                                    .finish(),
                            flow: kid_flow,
                        };
                        opt_inline_block_splits.push(split)
                    }

                    // Push residual fragments.
                    fragment_accumulator.fragments.push_all(successors);
                    abs_descendants.push_descendants(kid_abs_descendants);
                }
                ConstructionItemConstructionResult(WhitespaceConstructionItem(whitespace_node,
                                                                              whitespace_style))
                        => {
                    // Instantiate the whitespace fragment.
                    let fragment_info = UnscannedTextFragment(UnscannedTextFragmentInfo::from_text(" ".to_owned()));
                    let fragment = Fragment::from_opaque_node_and_style(whitespace_node,
                                                                   whitespace_style.clone(),
                                                                   fragment_info);
                    fragment_accumulator.fragments.push(fragment, whitespace_style)
                }
                ConstructionItemConstructionResult(TableColumnFragmentConstructionItem(_)) => {
                    // TODO: Implement anonymous table objects for missing parents
                    // CSS 2.1 § 17.2.1, step 3-2
                }
            }
        }

        // Finally, make a new construction result.
        if opt_inline_block_splits.len() > 0 || fragment_accumulator.fragments.len() > 0
                || abs_descendants.len() > 0 {
            let construction_item = InlineFragmentsConstructionItem(InlineFragmentsConstructionResult {
                splits: opt_inline_block_splits,
                fragments: fragment_accumulator.finish(),
                abs_descendants: abs_descendants,
            });
            ConstructionItemConstructionResult(construction_item)
        } else {
            NoConstructionResult
        }
    }

    /// Creates an `InlineFragmentsConstructionResult` for replaced content. Replaced content doesn't
    /// render its children, so this just nukes a child's fragments and creates a `Fragment`.
    fn build_fragments_for_replaced_inline_content(&mut self, node: &ThreadSafeLayoutNode)
                                               -> ConstructionResult {
        for kid in node.children() {
            kid.set_flow_construction_result(NoConstructionResult)
        }

        // If this node is ignorable whitespace, bail out now.
        //
        // FIXME(#2001, pcwalton): Don't do this if there's padding or borders.
        if node.is_ignorable_whitespace() {
            let opaque_node = OpaqueNodeMethods::from_thread_safe_layout_node(node);
            return ConstructionItemConstructionResult(WhitespaceConstructionItem(
                opaque_node,
                node.style().clone()))
        }

        let mut fragments = InlineFragments::new();
        fragments.push(Fragment::new(self, node), node.style().clone());

        let construction_item = InlineFragmentsConstructionItem(InlineFragmentsConstructionResult {
            splits: Vec::new(),
            fragments: fragments,
            abs_descendants: Descendants::new(),
        });
        ConstructionItemConstructionResult(construction_item)
    }

    /// Builds one or more fragments for a node with `display: inline`. This yields an
    /// `InlineFragmentsConstructionResult`.
    fn build_fragments_for_inline(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        // Is this node replaced content?
        if !node.is_replaced_content() {
            // Go to a path that concatenates our kids' fragments.
            self.build_fragments_for_nonreplaced_inline_content(node)
        } else {
            // Otherwise, just nuke our kids' fragments, create our fragment if any, and be done
            // with it.
            self.build_fragments_for_replaced_inline_content(node)
        }
    }

    /// TableCaptionFlow is populated underneath TableWrapperFlow
    fn place_table_caption_under_table_wrapper(&mut self,
                                               table_wrapper_flow: &mut Box<Flow:Share>,
                                               node: &ThreadSafeLayoutNode) {
        for kid in node.children() {
            match kid.swap_out_construction_result() {
                NoConstructionResult | ConstructionItemConstructionResult(_) => {}
                FlowConstructionResult(kid_flow, _) => {
                    // Only kid flows with table-caption are matched here.
                    assert!(kid_flow.is_table_caption());
                    table_wrapper_flow.add_new_child(kid_flow);
                }
            }
        }
    }

    /// Generates an anonymous table flow according to CSS 2.1 § 17.2.1, step 2.
    /// If necessary, generate recursively another anonymous table flow.
    fn generate_anonymous_missing_child(&mut self,
                                        child_flows: Vec<Box<Flow:Share>>,
                                        flow: &mut Box<Flow:Share>,
                                        node: &ThreadSafeLayoutNode) {
        let mut anonymous_flow = flow.generate_missing_child_flow(node);
        let mut consecutive_siblings = vec!();
        for kid_flow in child_flows.move_iter() {
            if anonymous_flow.need_anonymous_flow(kid_flow) {
                consecutive_siblings.push(kid_flow);
                continue;
            }
            if !consecutive_siblings.is_empty() {
                self.generate_anonymous_missing_child(consecutive_siblings, &mut anonymous_flow, node);
                consecutive_siblings = vec!();
            }
            anonymous_flow.add_new_child(kid_flow);
        }
        if !consecutive_siblings.is_empty() {
            self.generate_anonymous_missing_child(consecutive_siblings, &mut anonymous_flow, node);
        }
        // The flow is done.
        anonymous_flow.finish(self.layout_context);
        flow.add_new_child(anonymous_flow);
    }

    /// Builds a flow for a node with `display: table`. This yields a `TableWrapperFlow` with possibly
    /// other `TableCaptionFlow`s or `TableFlow`s underneath it.
    fn build_flow_for_table_wrapper(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let fragment = Fragment::new_from_specific_info(node, TableWrapperFragment);
        let mut wrapper_flow = box TableWrapperFlow::from_node_and_fragment(node, fragment) as Box<Flow:Share>;

        let table_fragment = Fragment::new_from_specific_info(node, TableFragment);
        let table_flow = box TableFlow::from_node_and_fragment(node, table_fragment) as Box<Flow:Share>;

        // We first populate the TableFlow with other flows than TableCaptionFlow.
        // We then populate the TableWrapperFlow with TableCaptionFlow, and attach
        // the TableFlow to the TableWrapperFlow
        let construction_result = self.build_flow_using_children(table_flow, node);
        self.place_table_caption_under_table_wrapper(&mut wrapper_flow, node);

        let mut abs_descendants = Descendants::new();
        let mut fixed_descendants = Descendants::new();

        // NOTE: The order of captions and table are not the same order as in the DOM tree.
        // All caption blocks are placed before the table flow
        match construction_result {
            FlowConstructionResult(table_flow, table_abs_descendants) => {
                wrapper_flow.add_new_child(table_flow);
                abs_descendants.push_descendants(table_abs_descendants);
            }
            _ => {}
        }

        // The flow is done.
        wrapper_flow.finish(self.layout_context);
        let is_positioned = wrapper_flow.as_block().is_positioned();
        let is_fixed_positioned = wrapper_flow.as_block().is_fixed();
        let is_absolutely_positioned = wrapper_flow.as_block().is_absolutely_positioned();
        if is_positioned {
            // This is the CB for all the absolute descendants.
            wrapper_flow.set_abs_descendants(abs_descendants);
            abs_descendants = Descendants::new();

            if is_fixed_positioned {
                // Send itself along with the other fixed descendants.
                fixed_descendants.push(Rawlink::some(wrapper_flow));
            } else if is_absolutely_positioned {
                // This is now the only absolute flow in the subtree which hasn't yet
                // reached its CB.
                abs_descendants.push(Rawlink::some(wrapper_flow));
            }
        }
        FlowConstructionResult(wrapper_flow, abs_descendants)
    }

    /// Builds a flow for a node with `display: table-caption`. This yields a `TableCaptionFlow`
    /// with possibly other `BlockFlow`s or `InlineFlow`s underneath it.
    fn build_flow_for_table_caption(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let flow = box TableCaptionFlow::from_node(self, node) as Box<Flow:Share>;
        self.build_flow_using_children(flow, node)
    }

    /// Builds a flow for a node with `display: table-row-group`. This yields a `TableRowGroupFlow`
    /// with possibly other `TableRowFlow`s underneath it.
    fn build_flow_for_table_rowgroup(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let fragment = Fragment::new_from_specific_info(node, TableRowFragment);
        let flow = box TableRowGroupFlow::from_node_and_fragment(node, fragment) as Box<Flow:Share>;
        self.build_flow_using_children(flow, node)
    }

    /// Builds a flow for a node with `display: table-row`. This yields a `TableRowFlow` with
    /// possibly other `TableCellFlow`s underneath it.
    fn build_flow_for_table_row(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let fragment = Fragment::new_from_specific_info(node, TableRowFragment);
        let flow = box TableRowFlow::from_node_and_fragment(node, fragment) as Box<Flow:Share>;
        self.build_flow_using_children(flow, node)
    }

    /// Builds a flow for a node with `display: table-cell`. This yields a `TableCellFlow` with
    /// possibly other `BlockFlow`s or `InlineFlow`s underneath it.
    fn build_flow_for_table_cell(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let fragment = Fragment::new_from_specific_info(node, TableCellFragment);
        let flow = box TableCellFlow::from_node_and_fragment(node, fragment) as Box<Flow:Share>;
        self.build_flow_using_children(flow, node)
    }

    /// Creates a fragment for a node with `display: table-column`.
    fn build_fragments_for_table_column(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        // CSS 2.1 § 17.2.1. Treat all child fragments of a `table-column` as `display: none`.
        for kid in node.children() {
            kid.set_flow_construction_result(NoConstructionResult)
        }

        let specific = TableColumnFragment(TableColumnFragmentInfo::new(node));
        let construction_item = TableColumnFragmentConstructionItem(
            Fragment::new_from_specific_info(node, specific)
        );
        ConstructionItemConstructionResult(construction_item)
    }

    /// Builds a flow for a node with `display: table-column-group`.
    /// This yields a `TableColGroupFlow`.
    fn build_flow_for_table_colgroup(&mut self, node: &ThreadSafeLayoutNode) -> ConstructionResult {
        let fragment = Fragment::new_from_specific_info(node,
                                               TableColumnFragment(TableColumnFragmentInfo::new(node)));
        let mut col_fragments = vec!();
        for kid in node.children() {
            // CSS 2.1 § 17.2.1. Treat all non-column child fragments of `table-column-group`
            // as `display: none`.
            match kid.swap_out_construction_result() {
                ConstructionItemConstructionResult(TableColumnFragmentConstructionItem(fragment)) => {
                    col_fragments.push(fragment);
                }
                _ => {}
            }
        }
        if col_fragments.is_empty() {
            debug!("add TableColumnFragment for empty colgroup");
            let specific = TableColumnFragment(TableColumnFragmentInfo::new(node));
            col_fragments.push(Fragment::new_from_specific_info(node, specific));
        }
        let mut flow = box TableColGroupFlow::from_node_and_fragments(node, fragment, col_fragments) as Box<Flow:Share>;
        flow.finish(self.layout_context);

        FlowConstructionResult(flow, Descendants::new())
    }
}

impl<'a> PostorderNodeMutTraversal for FlowConstructor<'a> {
    // Construct Flow based on 'display', 'position', and 'float' values.
    //
    // CSS 2.1 Section 9.7
    //
    // TODO: This should actually consult the table in that section to get the
    // final computed value for 'display'.
    //
    // `#[inline(always)]` because this is always called from the traversal function and for some
    // reason LLVM's inlining heuristics go awry here.
    #[inline(always)]
    fn process(&mut self, node: &ThreadSafeLayoutNode) -> bool {
        // Get the `display` property for this node, and determine whether this node is floated.
        let (display, float, positioning) = match node.type_id() {
            None => {
                // Pseudo-element.
                let style = node.style();
                (display::inline, style.get_box().float, style.get_box().position)
            }
            Some(ElementNodeTypeId(_)) => {
                let style = node.style();
                (style.get_box().display, style.get_box().float, style.get_box().position)
            }
            Some(TextNodeTypeId) => (display::inline, float::none, position::static_),
            Some(CommentNodeTypeId) |
            Some(DoctypeNodeTypeId) |
            Some(DocumentFragmentNodeTypeId) |
            Some(DocumentNodeTypeId) |
            Some(ProcessingInstructionNodeTypeId) => {
                (display::none, float::none, position::static_)
            }
        };

        debug!("building flow for node: {:?} {:?}", display, float);

        // Switch on display and floatedness.
        match (display, float, positioning) {
            // `display: none` contributes no flow construction result. Nuke the flow construction
            // results of children.
            (display::none, _, _) => {
                for child in node.children() {
                    let mut old_result = child.swap_out_construction_result();
                    old_result.destroy()
                }
            }

            // Table items contribute table flow construction results.
            (display::table, _, _) => {
                let construction_result = self.build_flow_for_table_wrapper(node);
                node.set_flow_construction_result(construction_result)
            }

            // Absolutely positioned elements will have computed value of
            // `float` as 'none' and `display` as per the table.
            // Only match here for block items. If an item is absolutely
            // positioned, but inline we shouldn't try to construct a block
            // flow here - instead, let it match the inline case
            // below.
            (display::block, _, position::absolute) | (_, _, position::fixed) => {
                node.set_flow_construction_result(self.build_flow_for_block(node))
            }

            // Inline items contribute inline fragment construction results.
            (display::inline, float::none, _) => {
                let construction_result = self.build_fragments_for_inline(node);
                node.set_flow_construction_result(construction_result)
            }

            // Table items contribute table flow construction results.
            (display::table_caption, _, _) => {
                let construction_result = self.build_flow_for_table_caption(node);
                node.set_flow_construction_result(construction_result)
            }

            // Table items contribute table flow construction results.
            (display::table_column_group, _, _) => {
                let construction_result = self.build_flow_for_table_colgroup(node);
                node.set_flow_construction_result(construction_result)
            }

            // Table items contribute table flow construction results.
            (display::table_column, _, _) => {
                let construction_result = self.build_fragments_for_table_column(node);
                node.set_flow_construction_result(construction_result)
            }

            // Table items contribute table flow construction results.
            (display::table_row_group, _, _) | (display::table_header_group, _, _) |
            (display::table_footer_group, _, _) => {
                let construction_result = self.build_flow_for_table_rowgroup(node);
                node.set_flow_construction_result(construction_result)
            }

            // Table items contribute table flow construction results.
            (display::table_row, _, _) => {
                let construction_result = self.build_flow_for_table_row(node);
                node.set_flow_construction_result(construction_result)
            }

            // Table items contribute table flow construction results.
            (display::table_cell, _, _) => {
                let construction_result = self.build_flow_for_table_cell(node);
                node.set_flow_construction_result(construction_result)
            }

            // Block flows that are not floated contribute block flow construction results.
            //
            // TODO(pcwalton): Make this only trigger for blocks and handle the other `display`
            // properties separately.

            (_, float::none, _) => {
                node.set_flow_construction_result(self.build_flow_for_block(node))
            }

            // Floated flows contribute float flow construction results.
            (_, float_value, _) => {
                let float_kind = FloatKind::from_property(float_value);
                node.set_flow_construction_result(
                    self.build_flow_for_floated_block(node, float_kind))
            }
        }

        true
    }
}

/// A utility trait with some useful methods for node queries.
trait NodeUtils {
    /// Returns true if this node doesn't render its kids and false otherwise.
    fn is_replaced_content(&self) -> bool;

    /// Returns true if this node is ignorable whitespace.
    fn is_ignorable_whitespace(&self) -> bool;

    /// Sets the construction result of a flow.
    fn set_flow_construction_result(&self, result: ConstructionResult);

    /// Replaces the flow construction result in a node with `NoConstructionResult` and returns the
    /// old value.
    fn swap_out_construction_result(&self) -> ConstructionResult;
}

impl<'ln> NodeUtils for ThreadSafeLayoutNode<'ln> {
    fn is_replaced_content(&self) -> bool {
        match self.type_id() {
            Some(TextNodeTypeId) |
            Some(ProcessingInstructionNodeTypeId) |
            Some(CommentNodeTypeId) |
            Some(DoctypeNodeTypeId) |
            Some(DocumentFragmentNodeTypeId) |
            Some(DocumentNodeTypeId) |
            None |
            Some(ElementNodeTypeId(HTMLImageElementTypeId)) => true,
            Some(ElementNodeTypeId(HTMLObjectElementTypeId)) => self.has_object_data(),
            Some(ElementNodeTypeId(_)) => false,
        }
    }

    fn is_ignorable_whitespace(&self) -> bool {
        match self.type_id() {
            Some(TextNodeTypeId) => {
                unsafe {
                    let text: JS<Text> = self.get_jsmanaged().transmute_copy();
                    if !is_whitespace((*text.unsafe_get()).characterdata.data) {
                        return false
                    }

                    // NB: See the rules for `white-space` here:
                    //
                    //    http://www.w3.org/TR/CSS21/text.html#propdef-white-space
                    //
                    // If you implement other values for this property, you will almost certainly
                    // want to update this check.
                    match self.style().get_inheritedtext().white_space {
                        white_space::normal => true,
                        _ => false,
                    }
                }
            }
            _ => false
        }
    }

    #[inline(always)]
    fn set_flow_construction_result(&self, result: ConstructionResult) {
        let mut layout_data_ref = self.mutate_layout_data();
        match &mut *layout_data_ref {
            &Some(ref mut layout_data) =>{
                match self.get_pseudo_element_type() {
                    Before | BeforeBlock => {
                        layout_data.data.before_flow_construction_result = result
                    },
                    After | AfterBlock => {
                        layout_data.data.after_flow_construction_result = result
                    },
                    Normal => layout_data.data.flow_construction_result = result,
                }
            },
            &None => fail!("no layout data"),
        }
    }

    #[inline(always)]
    fn swap_out_construction_result(&self) -> ConstructionResult {
        let mut layout_data_ref = self.mutate_layout_data();
        match &mut *layout_data_ref {
            &Some(ref mut layout_data) => {
                match self.get_pseudo_element_type() {
                    Before | BeforeBlock => {
                        mem::replace(&mut layout_data.data.before_flow_construction_result,
                                     NoConstructionResult)
                    }
                    After | AfterBlock => {
                        mem::replace(&mut layout_data.data.after_flow_construction_result,
                                     NoConstructionResult)
                    }
                    Normal => {
                        mem::replace(&mut layout_data.data.flow_construction_result,
                                     NoConstructionResult)
                    }
                }
            }
            &None => fail!("no layout data"),
        }
    }
}

/// Methods for interacting with HTMLObjectElement nodes
trait ObjectElement {
    /// Returns None if this node is not matching attributes.
    fn get_type_and_data(&self) -> (Option<&'static str>, Option<&'static str>);

    /// Returns true if this node has object data that is correct uri.
    fn has_object_data(&self) -> bool;

    /// Returns the "data" attribute value parsed as a URL
    fn get_object_data(&self, base_url: &Url) -> Option<Url>;
}

impl<'ln> ObjectElement for ThreadSafeLayoutNode<'ln> {
    fn get_type_and_data(&self) -> (Option<&'static str>, Option<&'static str>) {
        let elem = self.as_element();
        (elem.get_attr(&namespace::Null, "type"), elem.get_attr(&namespace::Null, "data"))
    }

    fn has_object_data(&self) -> bool {
        match self.get_type_and_data() {
            (None, Some(uri)) => is_image_data(uri),
            _ => false
        }
    }

    fn get_object_data(&self, base_url: &Url) -> Option<Url> {
        match self.get_type_and_data() {
            (None, Some(uri)) if is_image_data(uri) => Some(parse_url(uri, Some(base_url.clone()))),
            _ => None
        }
    }
}

/// Strips ignorable whitespace from the start of a list of fragments.
fn strip_ignorable_whitespace_from_start(fragments: &mut InlineFragments) {
    if fragments.len() == 0 {
        return
    }

    let InlineFragments {
        fragments: old_fragments,
        map: mut map
    } = mem::replace(fragments, InlineFragments::new());

    // FIXME(#2264, pcwalton): This is slow because vector shift is broken. :(
    let mut found_nonwhitespace = false;
    let mut new_fragments = Vec::new();
    for fragment in old_fragments.iter() {
        if !found_nonwhitespace && fragment.is_whitespace_only() {
            debug!("stripping ignorable whitespace from start");
            continue
        }

        found_nonwhitespace = true;
        new_fragments.push(fragment.clone())
    }

    map.fixup(old_fragments.as_slice(), new_fragments.as_slice());
    *fragments = InlineFragments {
        fragments: new_fragments,
        map: map,
    }
}

/// Strips ignorable whitespace from the end of a list of fragments.
fn strip_ignorable_whitespace_from_end(fragments: &mut InlineFragments) {
    if fragments.len() == 0 {
        return
    }

    let InlineFragments {
        fragments: old_fragments,
        map: mut map
    } = mem::replace(fragments, InlineFragments::new());

    let mut new_fragments = old_fragments.clone();
    while new_fragments.len() > 0 && new_fragments.as_slice().last().get_ref().is_whitespace_only() {
        debug!("stripping ignorable whitespace from end");
        drop(new_fragments.pop());
    }

    map.fixup(old_fragments.as_slice(), new_fragments.as_slice());
    *fragments = InlineFragments {
        fragments: new_fragments,
        map: map,
    }
}
