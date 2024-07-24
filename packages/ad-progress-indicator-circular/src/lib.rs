use customelement::{add_class_svg, inject_style, CustomElement,};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node, SvgElement};
// The boring part: a basic DOM component
struct ProgressIndicatorComponent {
    element: HtmlElement,
}

impl ProgressIndicatorComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let htmltemplate: String = include_str!("ad-progress-indicator-circular.html").into();
        console_log!("{}", "html");

        let element = customelement::create_element_from_html(
            &htmltemplate.as_str(),
            "ad-progress-indicator-circular",
        )
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

impl Default for ProgressIndicatorComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ProgressIndicatorComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-progress-indicator-circular.css").into();
        let css_one =
            include_str!("../../../tokens/css/components/circular-progress-indicator.css").into();

        inject_style(&this, css);
        inject_style(&this, css_one);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["indeterminate", "determinate"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        console_log!("{}", "attribute changed");
        if &name == "indeterminate" {
            let svg_element: SvgElement = self
                .element
                .query_selector(".svg")
                .expect("REASON")
                .expect("REASON")
                .dyn_into()
                .unwrap();
            add_class_svg(&svg_element, "indeterminate_svg");
            let active: SvgElement = self
                .element
                .query_selector(".circle")
                .expect("REASON")
                .expect("REASON")
                .dyn_into()
                .unwrap();
            add_class_svg(&active, "indeterminate_active_indicator");
        }

        if &name == "determinate" {
            let svg_element: SvgElement = self
                .element
                .query_selector(".svg")
                .expect("REASON")
                .expect("REASON")
                .dyn_into()
                .unwrap();
            add_class_svg(&svg_element, "determinate_svg");
            let active: SvgElement = self
                .element
                .query_selector(".circle")
                .expect("REASON")
                .expect("REASON")
                .dyn_into()
                .unwrap();
            add_class_svg(&active, "determinate_active_indicator");
        }

        // self::ProgressIndicatorComponent::update_style(&self);
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
    ProgressIndicatorComponent::define("ad-progress-indicator-circular");

    Ok(())
}
