use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct FilledFieldComponent {
    disabled: bool,
    element: HtmlElement,
}
impl FilledFieldComponent {
    fn new() -> Self {
        let disabled = false;
        let htmltemplate: String = include_str!("ad-filled-field.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-filled-field")
                .unwrap();
        Self { disabled, element }
    }

    fn view(&self) -> Node {
        let _el = &self.element;

        self.update_style();
        self.element.clone().into()
    }

    fn update_style(&self) {
        let _el = &self.element;

        if self.disabled == true {
            customelement::add_class(&_el, "disabled");
            self.element.set_attribute("tabindex", "-1").unwrap();
            let _icon_disabled: HtmlElement = self
                .element
                .query_selector(".trailing-icon")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            _icon_disabled.set_attribute("tabindex", "-1").unwrap();
        }
    }
}

impl Default for FilledFieldComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for FilledFieldComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_filled_field_css = include_str!("ad-filled-field.css").into();
        let comp_filled_field_css =
            include_str!("../../../tokens/css/components/filled-text-field.css").into();

        inject_style(&this, base_filled_field_css);
        inject_style(&this, comp_filled_field_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["label", "disabled", "error"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if &name == "disabled" {
            self.disabled = true;
        }

        if &name == "title" {
            let _text: HtmlElement = self
                .element
                .query_selector(".label-text")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            _text.set_inner_html(new_value.as_ref().unwrap());
        }

        if &name == "icon" {
            let _icon: HtmlElement = self
                .element
                .query_selector(".primary")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            customelement::add_class(&_icon, "leading");
        }

        if &name == "avatar" {
            let _icon: HtmlElement = self
                .element
                .query_selector(".primary")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            customelement::add_class(&_icon, "avatar");
        }

        self::FilledFieldComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    FilledFieldComponent::define("ad-filled-field");

    Ok(())
}
