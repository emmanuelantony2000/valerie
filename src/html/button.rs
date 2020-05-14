use crate::Component;
use core::ops::Deref;
use web_sys::{Element, Node};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::mpsc::Sender;

pub struct Button {
    view: Box<Element>,
}

impl Component for Button {}

impl Deref for Button {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl Button {
    pub fn view() -> Self {
        Self {
            view: {
                Box::new(
                    web_sys::window()
                        .expect("No global `window` exits")
                        .document()
                        .expect("Should have a document on window")
                        .create_element("button")
                        .expect("Cannot create `button`"),
                )
            },
        }
    }

    pub fn push_single(self, other: Box<dyn Deref<Target = Node>>) -> Self {
        self.view.append_child(&other).unwrap();
        self
    }

    pub fn push(self, others: Vec<Box<dyn Deref<Target = Node>>>) -> Self {
        for i in others {
            self.view.append_child(&i).unwrap();
        }

        self
    }

    pub fn value(self, content: &str) -> Self {
        self.view.set_inner_html(content);
        self
    }

    pub fn on_click(self, function: Box<dyn FnMut()>, sender: Sender<&'static str>) -> Self {
        let function = Closure::wrap(function as Box<dyn FnMut()>);

        let send = Box::new(move || sender.send("continue").unwrap()) as Box<dyn FnMut()>;
        let send = Closure::wrap(send);

        self.view.add_event_listener_with_callback("click", function.as_ref().unchecked_ref()).unwrap();
        self.view.add_event_listener_with_callback("click", send.as_ref().unchecked_ref()).unwrap();

        self
    }
}
