use customelement::{inject_style, CustomElement};

#[macro_use]
extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct IconComponent {
    element: HtmlElement,
    disabled: bool,
    checked: bool,
}

impl IconComponent {
    fn new() -> Self {
        console_log!("New");
        let disabled = false;
        let checked = false;
        let htmltemplate: String = include_str!("ad-toggle-switch.html").into();
        console_log!("html");
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-toggle-switch")
                .unwrap();
        console_log!("{}", element.inner_html());
        console_log!("element");
        Self { element, disabled, checked }
    }

    fn view(&self) -> Node {
        console_log!("view");
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        console_log!("updateStyle");
        let el = &self.element;
        console_log!("{}", el.tag_name().as_str());

        if self.disabled == true {
            self.element.query_selector("input").unwrap().unwrap().set_attribute("disabled", "").unwrap();
        }

        if self.checked == true {
            self.element.query_selector("input").unwrap().unwrap().set_attribute("checked", "").unwrap();
        }
    }
}

impl Default for IconComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for IconComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-toggle-switch.css").into();
        let css_one = include_str!("../../../tokens/css/components/switch.css").into();
        inject_style(&this, css);
        inject_style(&this, css_one);
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["disabled" , "checked"]
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_log!("connected_callback");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_log!("disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        console_log!("adopted");
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        console_log!("attribute changed");
        console_log!("{}", &name.as_str());

        if &name == "disabled" {
            self.disabled = true;
        }

        if &name == "checked" {
            self.checked = true;
        }
        self::IconComponent::updatestyle(&self);
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconComponent::define("ad-toggle-switch");

    Ok(())
}
