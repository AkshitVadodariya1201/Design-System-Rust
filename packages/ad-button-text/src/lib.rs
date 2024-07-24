use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ButtonTextComponent {
    disabled: bool,
    leadingicon: bool,
    trailingicon: bool,
    element: HtmlElement,
}
impl ButtonTextComponent {
    fn new() -> Self {
        let disabled = false;
        let leadingicon = false;
        let trailingicon = false;

        let htmltemplate: String = include_str!("ad-button-text.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-button-text")
                .unwrap();
        Self {
            disabled,
            element,
            leadingicon,
            trailingicon,
        }
    }

    fn view(&self) -> Node {
        let _el = &self.element;

        self.update_style();
        self.element.clone().into()
    }

    fn update_style(&self) {
        let _el = &self.element;

        if self.disabled == true {
            self.element.set_attribute("disabled", "").unwrap();
        }
        if self.leadingicon == true {
            customelement::add_class(&_el, "with-leading-icon");
        }
        if self.trailingicon == true {
            customelement::add_class(&_el, "with-trailing-icon");
        }
    }
}

impl Default for ButtonTextComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ButtonTextComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_button_text_css = include_str!("ad-button-text.css").into();
        let comp_button_text_css =
            include_str!("../../../tokens/css/components/text-button.css").into();

        inject_style(&this, base_button_text_css);
        inject_style(&this, comp_button_text_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["disabled", "leadingicon", "trailingicon"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        if &name == "disabled" {
            self.disabled = true;
        }
        if &name == "leadingicon" {
            self.leadingicon = true;
        }
        if &name == "trailingicon" {
            self.trailingicon = true;
        }

        self::ButtonTextComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ButtonTextComponent::define("ad-button-text");

    Ok(())
}
