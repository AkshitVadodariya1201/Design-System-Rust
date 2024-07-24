use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ButtonComponent {
    element: HtmlElement,
}
impl ButtonComponent {
    fn new() -> Self {
        console_log!("{}", "New");

        let htmltemplate: String = include_str!("ad-medium-top-app-bar.html").into();
        console_log!("{}", "html");
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-medium-top-app-bar")
                .unwrap();
        Self { element }
    }

    fn view(&self) -> Node {
        let el = &self.element;
        console_log!("{}", "view");
        console_log!("{}", el.tag_name().as_str());
        self.update_style();
        self.element.clone().into()
    }

    fn update_style(&self) {
        console_log!("{}", "updateStyle");
        let el = &self.element;
        console_log!("{}", el.tag_name().as_str());
    }

    // fn click() -> Result<(), JsValue> {
    //     console_log!("{}","click");
    //     Ok(())
    // }
}

impl Default for ButtonComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ButtonComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-medium-top-app-bar.css").into();
        inject_style(&this, css);
        // let css_one = include_str!("../../../tokens/css/components/top-app-bar.css").into();
        // inject_style(&this, css_one);
        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["title", "icon"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        console_log!("{}", "attribute changed");
        console_log!("{}", _this.tag_name().as_str());

        if &name == "title" {
            // self.element
            //     .set_attribute("aria-label", new_value.as_ref().unwrap())
            //     .unwrap();

            let _title = &self
                .element
                .query_selector(".headline")
                .unwrap()
                .unwrap()
                .set_inner_html(new_value.as_ref().unwrap());
        }

        if &name == "icon" {
            (&self.element)
                .query_selector(".leading-icon")
                .unwrap()
                .unwrap()
                .set_inner_html(new_value.as_ref().unwrap());
        }

        self::ButtonComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_log!("{}", "connected");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_log!("{}", "disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        console_log!("{}", "adopted");
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ButtonComponent::define("ad-medium-top-app-bar");

    Ok(())
}