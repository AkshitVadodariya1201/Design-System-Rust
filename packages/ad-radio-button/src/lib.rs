use customelement::{inject_style, CustomElement};
use wasm_bindgen::{JsValue};
// use js_sys::Function;
#[macro_use]
extern crate customelement;

use web_sys::{HtmlElement, Node};
// The boring part: a basic DOM component
struct ButtonComponent {
    element: HtmlElement,
    disabled: bool,
    checked: bool,
}
impl ButtonComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let disabled = false;
        let checked = false;
        let htmltemplate: String = include_str!("ad-radio-button.html").into();
        console_log!("{}", "html");
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "ad-radio-button")
                .unwrap();
        Self {
            disabled,
            checked,
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
        
        if self.disabled == true {
            self.element.first_element_child().expect("REASON").set_attribute("disabled", "").unwrap();
        }

        if self.checked == true {
            self.element.first_element_child().expect("REASON").set_attribute("checked", "").unwrap();
        }
    }
}

// fn click() -> Result<(), JsValue> {
//     console_log!("{}","click");
//     Ok(())
// }

impl Default for ButtonComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ButtonComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("ad-radio-button.css").into();
        let css_one = include_str!("../../../tokens/css/components/radio-button.css").into();
        inject_style(&this, css);
        inject_style(&this, css_one);
        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["title", "trailingicon", "disabled", "icon", "name", "checked"]
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


        if &name == "disabled" {
            self.disabled = true;
        }

        if &name == "checked" {
            self.checked = true;
        }

        self::ButtonComponent::update_style(&self);
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_log!("{}", "connected");
        // let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        //     self::ButtonComponent::click();
        // }) as Box<dyn FnMut(web_sys::Event)>);
        // let callback = Function::from(closure.as_ref().unchecked_ref::<JsValue>().into());
        // _this.add_event_listener_with_callback("click", &callback);
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
    ButtonComponent::define("ad-radio-button");

    Ok(())
}