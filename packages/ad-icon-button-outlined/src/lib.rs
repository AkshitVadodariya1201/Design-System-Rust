use customelement::{add_class, inject_style, CustomElement};

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct IconButtonOutlinedComponent {
    element: HtmlElement,
    disabled: bool,
    unselected: bool,
    selected: bool,
}

impl IconButtonOutlinedComponent {
    fn new() -> Self {
        let disabled = false;
        let unselected = false;
        let selected = false;

        let htmltemplate: String = include_str!("ad-icon-button-outlined.html").into();
        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-icon-button-outlined",
        )
        .unwrap();

        Self {
            element,
            disabled,
            unselected,
            selected,
        }
    }

    fn view(&self) -> Node {
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        let _el = &self.element;

        if self.unselected == true {
            add_class(&_el, "unselected");
        }
        if self.selected == true {
            add_class(&_el, "selected");
        }

        if self.disabled == true {
            self.element.set_attribute("disabled", "").unwrap();
        }
    }
}

impl Default for IconButtonOutlinedComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconButtonOutlinedComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_icon_button_outlined_css = include_str!("ad-icon-button-outlined.css").into();
        let comp_icon_button_outlined_css =
            include_str!("../../../tokens/css/components/outlined-icon-button.css").into();

        inject_style(&this, base_icon_button_outlined_css);
        inject_style(&this, comp_icon_button_outlined_css);

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

        self::IconButtonOutlinedComponent::updatestyle(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconButtonOutlinedComponent::define("ad-icon-button-outlined");

    Ok(())
}
