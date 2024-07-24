use customelement::{inject_style, CustomElement};

extern crate customelement;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Node};
struct ListItemComponent {
    oneline: bool,
    twoline: bool,
    threeline: bool,
    video: bool,
    disabled: bool,
    element: HtmlElement,
    avatar: bool,

}

impl ListItemComponent {
    fn new() -> Self {
        let oneline = false;
        let twoline = false;
        let threeline = false;
        let video = false;
        let disabled = false;
        let avatar = false;

        let htmltemplate: String = include_str!("ad-list-item.html").into();

        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-list-item")
                .unwrap();
        Self {
            element,
            oneline,
            twoline,
            threeline,
            video,
            disabled,
            avatar,

        }
    }

    fn view(&self) -> Node {
        self.updatestyle();
        self.element.clone().into()
    }

    fn updatestyle(&self) {
        let _el = &self.element;

        if self.oneline == true {
            customelement::add_class(&_el, "one-line");
        }

        if self.twoline == true {
            customelement::add_class(&_el, "two-line");
        }

        if self.threeline == true {
            customelement::add_class(&_el, "three-line");
            let _icon: HtmlElement = self
                .element
                .query_selector(".start")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            customelement::add_class(&_icon, "leading");
            let _trailing: HtmlElement = self
                .element
                .query_selector(".end")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            customelement::add_class(&_trailing, "trailing");
        }

        if self.video == true {
            let _video: HtmlElement = self
                .element
                .query_selector(".container")
                .unwrap()
                .clone()
                .expect("REASON")
                .dyn_into()
                .unwrap();
            customelement::add_class(&_video, "video");
        }

        if self.disabled == true {
            customelement::add_class(&_el, "disabled");
            self.element.set_attribute("tabindex", "-1").unwrap();
        }

        if self.avatar == true {
            
            customelement::add_class(&_el, "avatar");
        }

    }
}

impl Default for ListItemComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ListItemComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        let base_list_item_css = include_str!("ad-list-item.css").into();
        let comp_list_item_css = include_str!("../../../tokens/css/components/list.css").into();

        inject_style(&this, base_list_item_css);
        inject_style(&this, comp_list_item_css);

        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &[
            "supporting-text",
            "trailing-supporting-text",
            "headline",
            "one-line",
            "two-line",
            "three-line",
            "video",
            "disabled",
            "avatar",

        ]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        if &name == "supporting-text" {
            let _text = self
                .element
                .query_selector(".supporting-text")
                .unwrap()
                .unwrap();
            _text.set_inner_html(_new_value.as_ref().unwrap());
        }

        if &name == "trailing-supporting-text" {
            let _text = self
                .element
                .query_selector(".trailing-supporting-text")
                .unwrap()
                .unwrap();
            _text.set_inner_html(_new_value.as_ref().unwrap());
        }

        if &name == "headline" {
            let _text = self.element.query_selector(".headline").unwrap().unwrap();
            _text.set_inner_html(_new_value.as_ref().unwrap());
        }

        if &name == "one-line" {
            self.oneline = true;
        }

        if &name == "two-line" {
            self.twoline = true;
        }

        if &name == "three-line" {
            self.threeline = true;
        }

        if &name == "video" {
            self.video = true;
        }

        if &name == "disabled" {
            self.disabled = true;
        }

        if &name == "avatar" {
            self.avatar = true;
        }


        self::ListItemComponent::updatestyle(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {}

    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    fn adopted_callback(&mut self, _this: &HtmlElement) {}
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ListItemComponent::define("ad-list-item");

    Ok(())
}
