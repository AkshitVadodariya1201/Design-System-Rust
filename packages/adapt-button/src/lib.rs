use customelement::{inject_style, CustomElement};
use wasm_bindgen::JsValue;
// use js_sys::Function;
#[macro_use]
extern crate customelement;

use web_sys::{window, HtmlElement, Node, Text};
// The boring part: a basic DOM component
struct ButtonComponent {
    trailingicon: bool,
    icon: Text,
    typevalue: Text,
    element: HtmlElement,
    fullwidth: bool,
    buttonfilled: bool,
    grayfilled: bool,
    disabled: bool,
    title: Text,
}
impl ButtonComponent {
    fn new() -> Self {
        console_log!("{}", "New");
        let window = window().unwrap();
        let document = window.document().unwrap();
        let trailingicon = false;
        let disabled = false;
        let buttonfilled = false;
        let grayfilled = false;
        let fullwidth = false;
        let icon = document.create_text_node("");
        let typevalue = document.create_text_node("");
        let title = document.create_text_node("");
        // let element = document.create_element("p").
        // unwrap();
        let htmltemplate: String = include_str!("adapt-button.html").into();
        console_log!("{}", "html");
        // let mut handlebars = Handlebars::new();
        // handlebars.register_template_string("html_template", &htmltemplate);
        // let mut data = BTreeMap::new();
        // data.insert("outline", outline.data());
        // data.insert("raised", raised.data());
        // let htmlcontent = handlebars.render("html_template", &data);
        // convert htmlContent as template element
        let element =
            customelement::create_element_from_html(&htmltemplate.as_str(), "adapt-button")
                .unwrap();
        Self {
            trailingicon,
            disabled,
            fullwidth,
            buttonfilled,
            grayfilled,
            icon,
            typevalue,
            title,
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
        if self.typevalue.data() == "outline" {
            customelement::add_class(&el, "outline");
        }
        if self.typevalue.data() == "raised" {
            customelement::add_class(&el, "raised");
        }
        if self.typevalue.data() == "unelevated" {
            customelement::add_class(&el, "unelevated");
        }
        if self.typevalue.data() == "dense" {
            customelement::add_class(&el, "dense");
        }
        if self.buttonfilled == true {
            customelement::add_class(&el, "buttonfilled");
        }
        if self.fullwidth == true {
            customelement::add_class(&el, "fullwidth");
        }
        if self.disabled == true {
            customelement::add_class(&el, "disabled");
        }
        if self.grayfilled == true {
            customelement::add_class(&el, "grayfilled");
        }
    }

    // fn click() -> Result<(), JsValue> {
    //     console_log!("{}","click");
    //     Ok(())
    // }
}

impl Default for ButtonComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ButtonComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        console_log!("injecting children");
        let css = include_str!("adapt-button.css").into();
        inject_style(&this, css);
        let node = self.view();
        this.append_child(&node).unwrap();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &[
            "title",
            "trailingicon",
            "disabled",
            "buttonfilled",
            "fullwidth",
            "grayfilled",
            "icon",
            "type",
        ]
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
        if &name == "buttonfilled" {
            self.buttonfilled = true;
        }
        if &name == "fullwidth" {
            self.fullwidth = true;
        }
        if &name == "disabled" {
            self.disabled = true;
        }
        if &name == "grayfilled" {
            self.grayfilled = true;
        }
        if &name == "trailingicon" {
            console_log!("{}", "trailing icon");
            self.trailingicon = true;
        }
        if &name == "title" {
            let newhtml;
            let icon_name = self.icon.data();
            console_log!("Trailing icon {}", self.trailingicon);
            if icon_name != "" {
                let icon = format!("{}{}{}", "<adapt-icon>", icon_name, "</adapt-icon>");
                if self.trailingicon == true {
                    newhtml = format!("{}{}", new_value.as_ref().unwrap(), icon);
                } else {
                    newhtml = format!("{}{}", icon, new_value.as_ref().unwrap());
                }
            } else {
                newhtml = new_value.as_ref().unwrap().to_string();
            }
            self.element
                .set_attribute("aria-label", new_value.as_ref().unwrap().as_str())
                .expect("Failed to set aria-label attribute");

            self.element.set_inner_html(&newhtml);
        }
        if &name == "icon" {
            let newhtml = format!(
                "{}{}{}",
                "<adapt-icon>",
                new_value.as_ref().unwrap(),
                "</adapt-icon>"
            );
            self.element.set_inner_html(&newhtml);
            self.icon.set_data(new_value.as_ref().unwrap());
            let titlevalue = self.title.data();
            if titlevalue == "" {
                self.element
                    .set_attribute("aria-label", new_value.as_ref().unwrap().as_str())
                    .expect("Failed to set aria-label attribute");
            }
        }
        if &name == "type" {
            console_log!("Type {}", new_value.as_ref().unwrap().as_str());
            self.typevalue
                .set_data(new_value.as_ref().unwrap().as_str());
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
    ButtonComponent::define("adapt-button");

    Ok(())
}
