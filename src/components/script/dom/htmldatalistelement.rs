/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::BindingDeclarations::HTMLDataListElementBinding;
use dom::bindings::codegen::InheritTypes::{HTMLDataListElementDerived, NodeCast};
use dom::bindings::js::{JS, JSRef, RootCollection, Unrooted};
use dom::document::Document;
use dom::element::{Element, HTMLDataListElementTypeId};
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlcollection::{HTMLCollection, CollectionFilter};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, ElementNodeTypeId, window_from_node};
use servo_util::str::DOMString;

#[deriving(Encodable)]
pub struct HTMLDataListElement {
    pub htmlelement: HTMLElement
}

impl HTMLDataListElementDerived for EventTarget {
    fn is_htmldatalistelement(&self) -> bool {
        match self.type_id {
            NodeTargetTypeId(ElementNodeTypeId(HTMLDataListElementTypeId)) => true,
            _ => false
        }
    }
}

impl HTMLDataListElement {
    pub fn new_inherited(localName: DOMString, document: JS<Document>) -> HTMLDataListElement {
        HTMLDataListElement {
            htmlelement: HTMLElement::new_inherited(HTMLDataListElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: &JSRef<Document>) -> Unrooted<HTMLDataListElement> {
        let element = HTMLDataListElement::new_inherited(localName, document.unrooted());
        Node::reflect_node(~element, document, HTMLDataListElementBinding::Wrap)
    }
}

pub trait HTMLDataListElementMethods {
    fn Options(&self, abstract_self: &JSRef<HTMLDataListElement>) -> Unrooted<HTMLCollection>;
}

impl<'a> HTMLDataListElementMethods for JSRef<'a, HTMLDataListElement> {
    fn Options(&self, abstract_self: &JSRef<HTMLDataListElement>) -> Unrooted<HTMLCollection> {
        struct HTMLDataListOptionsFilter;
        impl CollectionFilter for HTMLDataListOptionsFilter {
            fn filter(&self, elem: &JSRef<Element>, _root: &JSRef<Node>) -> bool {
                elem.get().local_name == ~"option"
            }
        }
        let roots = RootCollection::new();
        let node: &JSRef<Node> = NodeCast::from_ref(abstract_self);
        let filter = ~HTMLDataListOptionsFilter;
        let window = window_from_node(node).root(&roots);
        HTMLCollection::create(&*window, node, filter)
    }
}

