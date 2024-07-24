use customelement::{inject_style, CustomElement};

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct DialogComponent {
    element: HtmlElement,

    stacked: bool,

}

impl DialogComponent {
    fn new() -> Self {
        let stacked = false;

        let htmltemplate: String = include_str!("ad-dialog.html").into();
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-dialog").unwrap();
        Self { element, stacked }

    }

    fn view(&self) -> Node {
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        let _el = &self.element;

        if self.stacked == true {
            customelement::add_class(_el, "stacked");
        }

    }
}

impl Default for DialogComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for DialogComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_dialog_css = include_str!("ad-dialog.css").into();
        let comp_dialog_css = include_str!("../../../tokens/css/components/dialog.css").into();

        inject_style(&this, base_dialog_css);
        inject_style(&this, comp_dialog_css);

        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["stacked"]

    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {

        if _name == "stacked" {
            self.stacked = true;
        }

        self::DialogComponent::updatestyle(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    DialogComponent::define("ad-dialog");

    Ok(())
}
