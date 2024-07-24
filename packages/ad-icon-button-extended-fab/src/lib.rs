use customelement::{add_class, inject_style, CustomElement};

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct IconButtonExtendedFabComponent {
    element: HtmlElement,
    primary: bool,
    surface: bool,
    secondary: bool,
    tertiary: bool,
    icon: bool,
}

impl IconButtonExtendedFabComponent {
    fn new() -> Self {
        let primary = false;
        let surface = false;
        let secondary = false;
        let tertiary = false;
        let icon = false;

        let htmltemplate: String = include_str!("ad-icon-button-extended-fab.html").into();
        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-icon-button-extended-fab",
        )
        .unwrap();

        Self {
            element,
            primary,
            surface,
            secondary,
            tertiary,
            icon,
        }
    }

    fn view(&self) -> Node {
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        let _el = &self.element;

        if self.icon {
            add_class(_el, "leading-icon");
        }

        if self.primary {
            add_class(_el, "primary");
        }

        if self.surface {
            add_class(_el, "surface");
        }

        if self.secondary {
            add_class(_el, "secondary");
        }

        if self.tertiary {
            add_class(_el, "tertiary");
        }
    }
}

impl Default for IconButtonExtendedFabComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconButtonExtendedFabComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_icon_button_extended_fab_css =
            include_str!("ad-icon-button-extended-fab.css").into();
        let comp_icon_button_extended_fab_css =
            include_str!("../../../tokens/css/components/extended-fab.css");

        inject_style(&this, base_icon_button_extended_fab_css);
        inject_style(&this, comp_icon_button_extended_fab_css.into());

        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["icon", "primary", "surface", "secondary", "tertiary"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        if &name == "icon" {
            self.icon = true;
        }

        if &name == "primary" {
            self.primary = true;
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

        self::IconButtonExtendedFabComponent::updatestyle(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconButtonExtendedFabComponent::define("ad-icon-button-extended-fab");

    Ok(())
}
