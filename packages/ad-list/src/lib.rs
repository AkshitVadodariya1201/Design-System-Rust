use customelement::{inject_style, CustomElement};

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct ListComponent {
    element: HtmlElement,
}

impl ListComponent {
    fn new() -> Self {
        let htmltemplate: String = include_str!("ad-list.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-list").unwrap();
        Self { element }
    }

    fn view(&self) -> Node {
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        let _el = &self.element;
    }
}

impl Default for ListComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ListComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_list_css = include_str!("ad-list.css").into();
        inject_style(&this, base_list_css);
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &[]
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        self::ListComponent::updatestyle(&self);
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ListComponent::define("ad-list");

    Ok(())
}
