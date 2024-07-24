use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};

#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, HtmlInputElement, Node};
// The boring part: a basic DOM component
struct CheckboxComponent {
    disabled: bool,
    checked: bool,
    error: bool,
    element: HtmlElement,
}
impl CheckboxComponent {
    fn new() -> Self {
        let disabled = false;
        let checked = false;
        let error = false;

        let htmltemplate: String = include_str!("ad-checkbox.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-checkbox").unwrap();
        Self {
            disabled,
            checked,
            error,
            element,
        }
    }

    fn view(&self) -> Node {
        let _el = &self.element;
        self.update_style();
        self.element.clone().into()
    }

    fn update_style(&self) {
        let el = &self.element;

        if self.disabled == true {
            let _input: HtmlElement = self
                .element
                .query_selector("input")
                .unwrap()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            _input.set_attribute("disabled", "").unwrap();
        }
        if self.checked == true {
            let _input: HtmlElement = self
                .element
                .query_selector("input")
                .unwrap()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            _input.set_attribute("checked", "").unwrap();
        }
        if self.error == true {
            customelement::add_class(&el, "error");
        }
    }
}

impl Default for CheckboxComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for CheckboxComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_checkbox_css = include_str!("ad-checkbox.css").into();
        let comp_checkbox_css = include_str!("../../../tokens/css/components/checkbox.css").into();

        inject_style(&this, base_checkbox_css);
        inject_style(&this, comp_checkbox_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["disabled", "checked", "error", "indeterminate"]
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
        if &name == "checked" {
            self.checked = true;
        }
        if &name == "error" {
            self.error = true;
        }
        if &name == "indeterminate" {
            let checkbox: Result<HtmlInputElement, web_sys::Element> = self
                .element
                .first_element_child()
                .unwrap()
                .dyn_into::<HtmlInputElement>();
            checkbox.unwrap().set_indeterminate(true);
        }

        self::CheckboxComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    CheckboxComponent::define("ad-checkbox");

    Ok(())
}
