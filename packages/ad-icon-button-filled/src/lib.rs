use customelement::{inject_style, CustomElement};

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct IconButtonFilledComponent {
    element: HtmlElement,
    disabled: bool,
    unselected: bool,
    selected: bool,
}

impl IconButtonFilledComponent {
    fn new() -> Self {
        let disabled = false;
        let unselected = false;
        let selected = false;

        let htmltemplate: String = include_str!("ad-icon-button-filled.html").into();
        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-icon-button-filled",
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

impl Default for IconButtonFilledComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconButtonFilledComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_icon_button_filled_css = include_str!("ad-icon-button-filled.css").into();
        let comp_icon_button_filled_css =
            include_str!("../../../tokens/css/components/filled-icon-button.css").into();

        inject_style(&this, base_icon_button_filled_css);
        inject_style(&this, comp_icon_button_filled_css);

        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["disabled", "selected", "unselected"]
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
        self::IconButtonFilledComponent::updatestyle(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconButtonFilledComponent::define("ad-icon-button-filled");

    Ok(())
}
