/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::AttrMethods;
use dom::bindings::codegen::BindingDeclarations::HTMLObjectElementBinding;
use dom::bindings::codegen::InheritTypes::HTMLObjectElementDerived;
use dom::bindings::codegen::InheritTypes::{ElementCast, HTMLElementCast};
use dom::bindings::js::{JS, JSRef, RootCollection, Unrooted};
use dom::bindings::error::ErrorResult;
use dom::document::Document;
use dom::element::{Element, HTMLObjectElementTypeId};
use dom::element::AttributeHandlers;
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlelement::HTMLElement;
use dom::htmlformelement::HTMLFormElement;
use dom::node::{Node, ElementNodeTypeId, NodeHelpers, window_from_node};
use dom::validitystate::ValidityState;
use dom::virtualmethods::VirtualMethods;
use dom::window::Window;
use servo_util::str::DOMString;

use servo_net::image_cache_task;
use servo_net::image_cache_task::ImageCacheTask;
use servo_util::url::parse_url;
use servo_util::namespace::Null;
use servo_util::url::is_image_data;
use url::Url;

#[deriving(Encodable)]
pub struct HTMLObjectElement {
    pub htmlelement: HTMLElement,
}

impl HTMLObjectElementDerived for EventTarget {
    fn is_htmlobjectelement(&self) -> bool {
        match self.type_id {
            NodeTargetTypeId(ElementNodeTypeId(HTMLObjectElementTypeId)) => true,
            _ => false
        }
    }
}

impl HTMLObjectElement {
    pub fn new_inherited(localName: DOMString, document: JS<Document>) -> HTMLObjectElement {
        HTMLObjectElement {
            htmlelement: HTMLElement::new_inherited(HTMLObjectElementTypeId, localName, document),
        }
    }

    pub fn new(localName: DOMString, document: &JSRef<Document>) -> Unrooted<HTMLObjectElement> {
        let element = HTMLObjectElement::new_inherited(localName, document.unrooted());
        Node::reflect_node(~element, document, HTMLObjectElementBinding::Wrap)
    }
}

trait ProcessDataURL {
    fn process_data_url(&mut self, image_cache: ImageCacheTask, url: Option<Url>);
}

impl<'a> ProcessDataURL for JSRef<'a, HTMLObjectElement> {
    // Makes the local `data` member match the status of the `data` attribute and starts
    /// prefetching the image. This method must be called after `data` is changed.
    fn process_data_url(&mut self, image_cache: ImageCacheTask, url: Option<Url>) {
        let roots = RootCollection::new();
        let elem: &JSRef<Element> = ElementCast::from_ref(self);

        // TODO: support other values
        match (elem.get_attribute(Null, "type").map(|x| x.root(&roots).Value()),
               elem.get_attribute(Null, "data").map(|x| x.root(&roots).Value())) {
            (None, Some(uri)) => {
                if is_image_data(uri) {
                    let data_url = parse_url(uri, url);
                    // Issue #84
                    image_cache.send(image_cache_task::Prefetch(data_url));
                }
            }
            _ => { }
        }
    }
}

pub trait HTMLObjectElementMethods {
    fn Data(&self) -> DOMString;
    fn SetData(&mut self, _data: DOMString) -> ErrorResult;
    fn Type(&self) -> DOMString;
    fn SetType(&mut self, _type: DOMString) -> ErrorResult;
    fn Name(&self) -> DOMString;
    fn SetName(&mut self, _name: DOMString) -> ErrorResult;
    fn UseMap(&self) -> DOMString;
    fn SetUseMap(&mut self, _use_map: DOMString) -> ErrorResult;
    fn GetForm(&self) -> Option<Unrooted<HTMLFormElement>>;
    fn Width(&self) -> DOMString;
    fn SetWidth(&mut self, _width: DOMString) -> ErrorResult;
    fn Height(&self) -> DOMString;
    fn SetHeight(&mut self, _height: DOMString) -> ErrorResult;
    fn GetContentDocument(&self) -> Option<Unrooted<Document>>;
    fn GetContentWindow(&self) -> Option<Unrooted<Window>>;
    fn WillValidate(&self) -> bool;
    fn Validity(&self) -> Unrooted<ValidityState>;
    fn ValidationMessage(&self) -> DOMString;
    fn CheckValidity(&self) -> bool;
    fn SetCustomValidity(&mut self, _error: DOMString);
    fn Align(&self) -> DOMString;
    fn SetAlign(&mut self, _align: DOMString) -> ErrorResult;
    fn Archive(&self) -> DOMString;
    fn SetArchive(&mut self, _archive: DOMString) -> ErrorResult;
    fn Code(&self) -> DOMString;
    fn SetCode(&mut self, _code: DOMString) -> ErrorResult;
    fn Declare(&self) -> bool;
    fn SetDeclare(&mut self, _declare: bool) -> ErrorResult;
    fn Hspace(&self) -> u32;
    fn SetHspace(&mut self, _hspace: u32) -> ErrorResult;
    fn Standby(&self) -> DOMString;
    fn SetStandby(&mut self, _standby: DOMString) -> ErrorResult;
    fn Vspace(&self) -> u32;
    fn SetVspace(&mut self, _vspace: u32) -> ErrorResult;
    fn CodeBase(&self) -> DOMString;
    fn SetCodeBase(&mut self, _codebase: DOMString) -> ErrorResult;
    fn CodeType(&self) -> DOMString;
    fn SetCodeType(&mut self, _codetype: DOMString) -> ErrorResult;
    fn Border(&self) -> DOMString;
    fn SetBorder(&mut self, _border: DOMString) -> ErrorResult;
    fn GetSVGDocument(&self) -> Option<Unrooted<Document>>;
}

impl<'a> HTMLObjectElementMethods for JSRef<'a, HTMLObjectElement> {
    fn Data(&self) -> DOMString {
        ~""
    }

    fn SetData(&mut self, _data: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Type(&self) -> DOMString {
        ~""
    }

    fn SetType(&mut self, _type: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Name(&self) -> DOMString {
        ~""
    }

    fn SetName(&mut self, _name: DOMString) -> ErrorResult {
        Ok(())
    }

    fn UseMap(&self) -> DOMString {
        ~""
    }

    fn SetUseMap(&mut self, _use_map: DOMString) -> ErrorResult {
        Ok(())
    }

    fn GetForm(&self) -> Option<Unrooted<HTMLFormElement>> {
        None
    }

    fn Width(&self) -> DOMString {
        ~""
    }

    fn SetWidth(&mut self, _width: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Height(&self) -> DOMString {
        ~""
    }

    fn SetHeight(&mut self, _height: DOMString) -> ErrorResult {
        Ok(())
    }

    fn GetContentDocument(&self) -> Option<Unrooted<Document>> {
        None
    }

    fn GetContentWindow(&self) -> Option<Unrooted<Window>> {
        None
    }

    fn WillValidate(&self) -> bool {
        false
    }

    fn Validity(&self) -> Unrooted<ValidityState> {
        let roots = RootCollection::new();
        let doc = self.htmlelement.element.node.owner_doc().root(&roots);
        let window = doc.deref().window.root(&roots);
        ValidityState::new(&window.root_ref())
    }

    fn ValidationMessage(&self) -> DOMString {
        ~""
    }

    fn CheckValidity(&self) -> bool {
        false
    }

    fn SetCustomValidity(&mut self, _error: DOMString) {
    }

    fn Align(&self) -> DOMString {
        ~""
    }

    fn SetAlign(&mut self, _align: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Archive(&self) -> DOMString {
        ~""
    }

    fn SetArchive(&mut self, _archive: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Code(&self) -> DOMString {
        ~""
    }

    fn SetCode(&mut self, _code: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Declare(&self) -> bool {
        false
    }

    fn SetDeclare(&mut self, _declare: bool) -> ErrorResult {
        Ok(())
    }

    fn Hspace(&self) -> u32 {
        0
    }

    fn SetHspace(&mut self, _hspace: u32) -> ErrorResult {
        Ok(())
    }

    fn Standby(&self) -> DOMString {
        ~""
    }

    fn SetStandby(&mut self, _standby: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Vspace(&self) -> u32 {
        0
    }

    fn SetVspace(&mut self, _vspace: u32) -> ErrorResult {
        Ok(())
    }

    fn CodeBase(&self) -> DOMString {
        ~""
    }

    fn SetCodeBase(&mut self, _codebase: DOMString) -> ErrorResult {
        Ok(())
    }

    fn CodeType(&self) -> DOMString {
        ~""
    }

    fn SetCodeType(&mut self, _codetype: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Border(&self) -> DOMString {
        ~""
    }

    fn SetBorder(&mut self, _border: DOMString) -> ErrorResult {
        Ok(())
    }

    fn GetSVGDocument(&self) -> Option<Unrooted<Document>> {
        None
    }
}

impl<'a> VirtualMethods for JSRef<'a, HTMLObjectElement> {
    fn super_type(&self) -> Option<~VirtualMethods:> {
        let htmlelement: &JSRef<HTMLElement> = HTMLElementCast::from_ref(self);
        Some(~htmlelement.clone() as ~VirtualMethods:)
    }

    fn after_set_attr(&mut self, name: DOMString, value: DOMString) {
        let roots = RootCollection::new();
        match self.super_type() {
            Some(ref mut s) => s.after_set_attr(name.clone(), value),
            _ => (),
        }

        if "data" == name {
            let window = window_from_node(self).root(&roots);
            let url = Some(window.get().get_url());
            self.process_data_url(window.get().image_cache_task.clone(), url);
        }
    }
}
