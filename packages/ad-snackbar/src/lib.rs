use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct SnackbarComponent {
    element: HtmlElement,
}
impl SnackbarComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let htmltemplate: String = include_str!("ad-snackbar.html").into();
        console_log!("{}", "html");

        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-snackbar",
        )
        .unwrap();
        Self {
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
    }

}

impl Default for SnackbarComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for SnackbarComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-snackbar.css").into();
        inject_style(&this, css);
        let css_one = include_str!("../../../tokens/css/components/snackbar.css").into();
        inject_style(&this, css_one);
        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["labeltext", "icon", "action"]
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

        if &name == "action" {
            let sec = self.element.query_selector(".button").unwrap();
            sec.expect("REASON").set_inner_html(new_value.as_ref().unwrap());
        }
        if &name == "labeltext" {
            
            let sec = self.element.query_selector(".label-text").unwrap();
            sec.expect("REASON").set_inner_html(new_value.as_ref().unwrap());
            console_log!("hello");

            // self.element.set_attribute("aria-label", new_value.as_ref().unwrap());

        }
        if &name == "icon" {
        
            let add_cl:HtmlElement = self.element.first_element_child().unwrap().dyn_into().unwrap();
            customelement::add_class(&add_cl, "state-layer-icon");
            
            let sec = self.element.query_selector(".material-icons").unwrap();
            sec.expect("REASON").set_inner_html(new_value.as_ref().unwrap());

            let sec = self.element.query_selector(".text-icon").unwrap();
            sec.expect("REASON").set_attribute("class", "with-icon").unwrap();
        }

        self::SnackbarComponent::update_style(&self);
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
    SnackbarComponent::define("ad-snackbar");

    Ok(())
}