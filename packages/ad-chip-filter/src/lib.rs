use customelement::{add_class, inject_style, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
#[macro_use]
extern crate customelement;

use web_sys::{window, HtmlElement, Node};
// The boring part: a basic DOM component
struct ChipFilterComponent {
    disabled: bool,
    elevated: bool,
    selected: bool,
    element: HtmlElement,
}
impl ChipFilterComponent {
    fn new() -> Self {
        let disabled = false;
        let elevated = false;
        let selected = false;
        let htmltemplate: String = include_str!("ad-chip-filter.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-chip-filter")
                .unwrap();
        Self {
            disabled,
            element,
            elevated,
            selected,
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
        if self.selected == true {
            customelement::add_class(&_el, "selected");
        }
    }
}

impl Default for ChipFilterComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ChipFilterComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let base_filter_chip_css = include_str!("ad-chip-filter.css").into();
        let comp_filter_chip_css =
            include_str!("../../../tokens/css/components/filter-chip.css").into();

        inject_style(&this, base_filter_chip_css);
        inject_style(&this, comp_filter_chip_css);

        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["title", "disabled", "elevated", "selected", "removable"]
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

        if &name == "selected" {
            self.selected = true;
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

        if &name == "removable" {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let _new_el = document.create_element("button").unwrap();
            self.element.append_child(&_new_el).unwrap();
            let _button: HtmlElement = _new_el.clone().dyn_into().unwrap();
            add_class(&_button, "icon");

            _new_el.set_inner_html(new_value.as_ref().unwrap());
            let _new_str = _new_el.outer_html().to_string();

            let html = format!("{}", "<slot name='trailing-icon'></slot>");
            _new_el.set_inner_html(&html);

            if self.disabled == true {
                _new_el.set_attribute("tabindex", "-1").unwrap();
            }
        }

        self::ChipFilterComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ChipFilterComponent::define("ad-chip-filter");

    Ok(())
}
