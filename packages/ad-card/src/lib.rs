use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct CardComponent {
    element: HtmlElement,
    elevated: bool,
    filled: bool,
    outline: bool,
    disabled: bool,
}
impl CardComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let elevated = false;
        let filled = false;
        let outline = false;
        let disabled = false;
        
        let htmltemplate: String = include_str!("ad-card.html").into();
        console_log!("{}", "html");
        
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-card")
                .unwrap();
        Self {
            element,
            elevated,
            filled,
            outline,
            disabled,
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
        if self.elevated == true {
            customelement::add_class(&el, "elevated");
        }
        if self.filled == true {
            customelement::add_class(&el, "filled");
        }
        if self.outline == true {
            customelement::add_class(&el, "outline");
        }
        if self.disabled == true {
            self.element.set_attribute("disabled", "").unwrap();       
            // customelement::add_class(&el, "disabled");
        }
    }

    // fn click() -> Result<(), JsValue> {
    //     console_log!("{}","click");
    //     Ok(())
    // }
}

impl Default for CardComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for CardComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-card.css").into();
        let css_one = include_str!("ad-card-elevated.css").into();
        let css_two: &str = include_str!("ad-card-filled.css").into();
        let css_three = include_str!("ad-card-outline.css").into();
        inject_style(&this, css);
        inject_style(&this, css_one);
        inject_style(&this, css_two);
        inject_style(&this, css_three);
        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &[
            "elevated",
            "filled",
            "outline",
            "disabled",
        ]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        console_log!("{}", "attribute changed");
        console_log!("{}", _this.tag_name().as_str());
        
        if &name == "elevated" {
            self.elevated = true;
        }
        if &name == "filled" {
            self.filled = true;
        }
        if &name == "outline" {
            self.outline = true;
        }
        if &name == "disabled" {
            self.disabled = true;
        }
        
        self::CardComponent::update_style(&self);
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
    CardComponent::define("ad-card");

    Ok(())
}