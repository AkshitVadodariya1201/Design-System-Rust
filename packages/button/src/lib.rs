use customelement::{inject_style, CustomElement};

#[macro_use]
extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, Node, Text};
struct MyWebComponent {
    outline: bool,
    sharp: Text,
    element: HtmlElement,
}

impl MyWebComponent {
    fn new() -> Self {
        console_log!("New");
        let window = window().unwrap();
        let document = window.document().unwrap();
        let outline = false;
        let sharp = document.create_text_node("");
        let htmltemplate: String = include_str!("my-button.html").into();
        console_log!("html");
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "my-button").unwrap();
        // console_log!("{}", element.inner_html());
        // console_log!("element");
        Self {
            outline,
            sharp,
            element,
        }
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
        if self.outline == true {
            customelement::add_class(&el, "outline");
        }
        if self.sharp.data() == "true" {
            customelement::add_class(&el, "sharp");
        }
    }
}

impl Default for MyWebComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for MyWebComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("my-button.css").into();
        inject_style(&this, css);
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["outline", "sharp", "title"]
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
        new_value: Option<String>,
    ) {
        console_log!("attribute changed");
        console_log!("{}", &name.as_str());
        if &name == "outline" {
            self.outline = true;
        }
        if &name == "sharp" {
            self.sharp.set_data("true");
            console_log!("{}", &self.sharp.data().as_str());
        }
        if &name == "title" {
            let html = self.element.inner_html();
            let newhtml = format!("{}{}", html, new_value.unwrap());
            self.element.set_inner_html(&newhtml);
        }
        self::MyWebComponent::updatestyle(&self);
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    MyWebComponent::define("my-button");

    Ok(())
}
