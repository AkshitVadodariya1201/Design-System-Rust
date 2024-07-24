use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct IconButtonFabComponent {
    small: bool,
    large: bool,
    surface: bool,
    secondary: bool,
    tertiary: bool,
    element: HtmlElement,
}
impl IconButtonFabComponent {
    fn new() -> Self {
        let small = false;
        let large = false;
        let surface = false;
        let secondary = false;
        let tertiary = false;

        let htmltemplate: String = include_str!("ad-icon-button-fab.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-icon-button-fab")
                .unwrap();
        Self {
            small,
            large,
            surface,
            secondary,
            tertiary,
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

        if self.small == true {
            customelement::add_class(&_el, "small");
        }
        if self.large == true {
            customelement::add_class(&_el, "large");
        }
        if self.surface == true {
            customelement::add_class(&_el, "surface");
        }
        if self.secondary == true {
            customelement::add_class(&_el, "secondary");
        }
        if self.tertiary == true {
            customelement::add_class(&_el, "tertiary");
        }
    }
}

impl Default for IconButtonFabComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconButtonFabComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_icon_button_fab_css = include_str!("ad-icon-button-fab.css").into();
        let comp_icon_button_fab_css =
            include_str!("../../../tokens/css/components/fab.css").into();

        inject_style(&this, base_icon_button_fab_css);
        inject_style(&this, comp_icon_button_fab_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["small", "large", "surface", "secondary", "tertiary"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        if &name == "small" {
            self.small = true;
        }
        if &name == "large" {
            self.large = true;
        }
        if &name == "surface" {
            self.surface = true;
        }
        if &name == "secondary" {
            self.secondary = true;
        }
        if &name == "tertiary" {
            self.tertiary = true;
        }

        self::IconButtonFabComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconButtonFabComponent::define("ad-icon-button-fab");

    Ok(())
}
