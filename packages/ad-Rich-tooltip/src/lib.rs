use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct TooltipComponent {
    element: HtmlElement,
}
impl TooltipComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let htmltemplate: String = include_str!("ad-rich-tooltip.html").into();
        console_log!("{}", "html");
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-rich-tooltip")
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

impl Default for TooltipComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for TooltipComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-rich-tooltip.css").into();
        inject_style(&this, css);
        let css_one = include_str!("../../../tokens/css/components/rich-tooltip.css").into();
        inject_style(&this, css_one);
        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["title", "labeltext"]
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
            let sec = self.element.query_selector(".label-text").unwrap();
            sec.expect("REASON")
                .set_inner_html(new_value.as_ref().unwrap());
        }

        if &name == "labeltext" {
            let sec = self.element.query_selector(".supporting-text").unwrap();
            sec.expect("REASON")
                .set_inner_html(new_value.as_ref().unwrap());
        }

        self::TooltipComponent::update_style(&self);
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
    TooltipComponent::define("ad-rich-tooltip");

    Ok(())
}
