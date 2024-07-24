use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsValue};
// use js_sys::Function;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ButtonElevatedComponent {
    element: HtmlElement,
    disabled: bool,
}
impl ButtonElevatedComponent {
    fn new() -> Self {
        let disabled = false;
        let htmltemplate: String = include_str!("ad-button-elevated.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-button-elevated")
                .unwrap();
        Self {
            disabled,
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
        if self.disabled == true {
            self.element.set_attribute("disabled", "").unwrap();
        }
    }

    // fn click() -> Result<(), JsValue> {
    //     console_log!("{}","click");
    //     Ok(())
    // }
}

impl Default for ButtonElevatedComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ButtonElevatedComponent {
    fn inject_children(&mut self, this: &HtmlElement) {

        let base_button_elevated_css = include_str!("ad-button-elevated.css").into();
        let comp_button_elevated_css = include_str!("../../../tokens/css/components/elevated-button.css").into();

        inject_style(&this, base_button_elevated_css);
        inject_style(&this, comp_button_elevated_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["disabled"]
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

        self::ButtonElevatedComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {

    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {

    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {

    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ButtonElevatedComponent::define("ad-button-elevated");

    Ok(())
}