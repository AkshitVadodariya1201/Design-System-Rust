use customelement::{inject_style, CustomElement};

#[macro_use]
extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct IconComponent {
    element: HtmlElement,
}

impl IconComponent {
    fn new() -> Self {
        console_log!("New");
        let htmltemplate: String = include_str!("ad-search.html").into();
        console_log!("html");
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-search").unwrap();
        console_log!("{}", element.inner_html());
        console_log!("element");
        Self { element }
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
        let css = include_str!("ad-search.css").into();
        let css_one = include_str!("../../../tokens/css/components/search-bar.css").into();
        inject_style(&this, css);
        inject_style(&this, css_one);
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["avatar", "icon"]
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

        if &name == "avatar" {
            let add_class1: HtmlElement = self.element.query_selector(".trailing-element").expect("REASON").expect("REASON").dyn_into().unwrap();
            customelement::add_class(&add_class1, "avatar");
        }

        if &name == "icon" {
            let add_class1: HtmlElement = self.element.query_selector(".trailing-element").expect("REASON").expect("REASON").dyn_into().unwrap();
            customelement::add_class(&add_class1, "material-icons");
            let add_class2: HtmlElement = self.element.query_selector(".trailing-element").expect("REASON").expect("REASON").dyn_into().unwrap();
            customelement::add_class(&add_class2, "trailing-icon");
        }
        self::IconComponent::updatestyle(&self);
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    IconComponent::define("ad-search");

    Ok(())
}
