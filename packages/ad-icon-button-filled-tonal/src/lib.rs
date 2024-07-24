use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};

struct IconButtonFilledTonalComponent {
    element: HtmlElement,
    disabled: bool,
    selected: bool,
    unselected: bool,
}

impl IconButtonFilledTonalComponent {
    fn new() -> Self {
        let disabled = false;
        let selected = false;
        let unselected = false;
        let htmltemplate: String = include_str!("ad-icon-button-filled-tonal.html").into();

        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-icon-button-filled-tonal",
        )
        .unwrap();

        Self {
            element,
            disabled,
            selected,
            unselected,
        }
    }

    fn view(&self) -> Node {
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        let _el = &self.element;

        if self.disabled == true {
            self.element.set_attribute("disabled", "").unwrap();
        }
        if self.unselected == true {
            customelement::add_class(&_el, "unselected");
        }
        if self.selected == true {
            customelement::add_class(&_el, "selected");
        }
    }
}

impl Default for IconButtonFilledTonalComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconButtonFilledTonalComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_icon_button_filled_tonal = include_str!("ad-icon-button-filled-tonal.css").into();
        let comp_icon_button_filled_tonal =
            include_str!("../../../tokens/css/components/filled-tonal-icon-button.css").into();

        inject_style(&this, base_icon_button_filled_tonal);
        inject_style(&this, comp_icon_button_filled_tonal);

        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["disabled", "unselected", "selected"]
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

        if &name == "unselected" {
            self.unselected = true;
        }

        if &name == "selected" {
            self.selected = true;
        }

        self::IconButtonFilledTonalComponent::updatestyle(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconButtonFilledTonalComponent::define("ad-icon-button-filled-tonal");

    Ok(())
}
