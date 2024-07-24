use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ChipAssistComponent {
    elevated: bool,
    disabled: bool,
    element: HtmlElement,
}
impl ChipAssistComponent {
    fn new() -> Self {

        let disabled = false;
        let elevated = false;
        let htmltemplate: String = include_str!("ad-chip-assist.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-chip-assist")
                .unwrap();
        Self {
            disabled,
            elevated,
            element,
        }
    }

    fn view(&self) -> Node {
        let _el = &self.element;

        self.update_style();
        self.element.clone().into()
    }

    fn update_style(&self) {
      
        let _el = &self.element;
       
        if self.disabled == true {
            customelement::add_class(&_el, "disabled");
            self.element.set_attribute("tabindex", "-1").unwrap();
        }
        if self.elevated == true {
            customelement::add_class(&_el, "elevated");
        }
    }
}

impl Default for ChipAssistComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ChipAssistComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
       
        let base_assist_chip_css = include_str!("ad-chip-assist.css").into();
        let comp_assist_chip_css = include_str!("../../../tokens/css/components/assist-chip.css").into();

        inject_style(&this, base_assist_chip_css);
        inject_style(&this, comp_assist_chip_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["title", "disabled", "elevated"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {

        if &name == "disabled" {
            self.disabled = true;
        }

        if &name == "elevated" {
            self.elevated = true;
        }

        if &name == "title" {
            let _text: HtmlElement = self
                .element
                .query_selector(".label-text")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            _text.set_inner_html(new_value.as_ref().unwrap());
        }

        self::ChipAssistComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ChipAssistComponent::define("ad-chip-assist");

    Ok(())
}
