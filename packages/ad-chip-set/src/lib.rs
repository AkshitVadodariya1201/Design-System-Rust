use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ChipSetComponent {
    scrollable: bool,
    element: HtmlElement,
}
impl ChipSetComponent {
    fn new() -> Self {
        let scrollable = false;
        let htmltemplate: String = include_str!("ad-chip-set.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-chip-set").unwrap();
        Self {
            scrollable,
            element,
        }
    }

    fn view(&self) -> Node {
        let _el = &self.element;

        self.update_style();
        self.element.clone().into()
    }

    fn update_style(&self) {
        let _el = &self.element;

        if self.scrollable == true {
            customelement::add_class(&_el, "scrollable");
        }
    }
}

impl Default for ChipSetComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ChipSetComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_input_chip_css = include_str!("ad-chip-set.css").into();

        inject_style(&this, base_input_chip_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["scrollable"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        if &name == "scrollable" {
            self.scrollable = true;
        }
        self::ChipSetComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ChipSetComponent::define("ad-chip-set");

    Ok(())
}
