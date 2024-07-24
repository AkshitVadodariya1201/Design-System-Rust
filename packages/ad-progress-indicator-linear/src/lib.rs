use customelement::{add_class, inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{window, HtmlElement, Node, Text};
// The boring part: a basic DOM component
struct ProgressIndicatorComponent {
    inderminate: bool,
    element: HtmlElement,
}
impl ProgressIndicatorComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let window = window().unwrap();
        let document = window.document().unwrap();
        let inderminate = false;
        let htmltemplate: String = include_str!("ad-progress-indicator-linear.html").into();
        console_log!("{}", "html");
        
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-progress-indicator-linear")
                .unwrap();
        Self {
            inderminate,
            element,
        }
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
        if self.inderminate == true {
            let el = 
            self.element.query_selector(".active_indicator").expect("REASON").expect("REASON").dyn_into().unwrap();
            customelement::add_class(&el, "inderminate");

        }
    }

    // fn click() -> Result<(), JsValue> {
    //     console_log!("{}","click");
    //     Ok(())
    // }
}

impl Default for ProgressIndicatorComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ProgressIndicatorComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-progress-indicator-linear.css").into();
        let css_one = include_str!("../../../tokens/css/components/linear-progress-indicator.css").into();

        inject_style(&this, css);
        inject_style(&this, css_one);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["inderminate"]
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

        if &name == "inderminate"{
            self.inderminate = true;
        }

        self::ProgressIndicatorComponent::update_style(&self);
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
    ProgressIndicatorComponent::define("ad-progress-indicator-linear");

    Ok(())
}