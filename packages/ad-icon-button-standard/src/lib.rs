use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;

#[macro_use]
extern crate customelement;
use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct IconButtonStandardComponent {
    element: HtmlElement,
    disabled: bool,
    unselected: bool,
    selected: bool,
}
impl IconButtonStandardComponent {
    fn new() -> Self {
        let disabled = false;
        let unselected = false;
        let selected = false;

        let htmltemplate: String = include_str!("ad-icon-button-standard.html").into();

        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-icon-button-standard",
        )
        .unwrap();
        Self {
            disabled,
            element,
            unselected,
            selected,
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
        if self.unselected == true {
            customelement::add_class(&_el, "unselected");
        }
        if self.selected == true {
            customelement::add_class(&_el, "selected");
        }
    }
}

impl Default for IconButtonStandardComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconButtonStandardComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_icon_button_standard_css = include_str!("ad-icon-button-standard.css").into();
        let comp_icon_button_standard_css =
            include_str!("../../../tokens/css/components/icon-button.css").into();

        inject_style(&this, base_icon_button_standard_css);
        inject_style(&this, comp_icon_button_standard_css);

        let node = self.view();
        this.append_child(&node).unwrap();
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

        self::IconButtonStandardComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconButtonStandardComponent::define("ad-icon-button-standard");

    Ok(())
}
