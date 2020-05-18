use crate::Component;
use alloc::boxed::Box;
use core::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};
use core::fmt::Display;
use alloc::string::ToString;

#[derive(Clone)]
pub struct Button {
    view: Element,
}

impl Component for Button {
    fn view(&mut self) -> Element {
        self.view.clone()
    }
}

impl Deref for Button {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl From<Button> for Element {
    fn from(x: Button) -> Self {
        x.view
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            view: {
                document()
                    .create_element("button")
                    .expect("Cannot create `button`")
            },
        }
    }

    pub fn push_single(self, other: Element) -> Self {
        self.view.append_child(&other).unwrap();
        self
    }

    pub fn push(self, others: &[Element]) -> Self {
        others.iter().for_each(|i| {
            self.append_child(i).unwrap();
        });

        self
    }

    pub fn value<T>(self, content: T) -> Self where T: Display {
        self.view.set_inner_html(content.to_string().as_ref());
        self
    }

    pub fn on_click(self, function: Box<dyn FnMut()>) -> Self {
        let function = Closure::wrap(function);

        self.view
            .add_event_listener_with_callback("click", function.as_ref().unchecked_ref())
            .unwrap();
        function.forget();

        self
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on window")
}
