use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ChipSuggestionComponent {
    elevated: bool,
    disabled: bool,
    element: HtmlElement,
}
impl ChipSuggestionComponent {
    fn new() -> Self {
        let disabled = false;
        let elevated = false;
        let htmltemplate: String = include_str!("ad-chip-suggestion.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-chip-suggestion")
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

impl Default for ChipSuggestionComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ChipSuggestionComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_suggestion_chip_css = include_str!("ad-chip-suggestion.css").into();
        let comp_suggestion_chip_css =
            include_str!("../../../tokens/css/components/suggestion-chip.css").into();

        inject_style(&this, base_suggestion_chip_css);
        inject_style(&this, comp_suggestion_chip_css);

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
        console_log!("{}", "attribute changed");
        console_log!("{}", _this.tag_name().as_str());

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

        self::ChipSuggestionComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ChipSuggestionComponent::define("ad-chip-suggestion");

    Ok(())
}
