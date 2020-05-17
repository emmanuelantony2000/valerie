use crate::Component;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::mem::transmute;
use core::ops::Deref;
use futures::channel::mpsc::Sender;
use futures::sink::SinkExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};
// use std::sync::mpsc::Sender;

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

impl Button {
    pub fn new() -> Self {
        Self {
            view: {
                web_sys::window()
                    .expect("No global `window` exits")
                    .document()
                    .expect("Should have a document on window")
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
        for i in others {
            self.view.append_child(&i).unwrap();
        }

        self
    }

    pub fn value(self, content: &str) -> Self {
        self.view.set_inner_html(content);
        self
    }

    // pub fn on_click(
    //     self,
    //     function: Box<dyn FnMut()>,
    //     mut tx: Sender<Option<&'static str>>,
    // ) -> Self {
    //     let function = Closure::wrap(function as Box<dyn FnMut()>);

    //     let send =
    //         Box::new(move || while tx.try_send(Some("iter")).is_err() {}) as Box<dyn FnMut()>;
    //     let send = Closure::wrap(send);

    //     self.view
    //         .add_event_listener_with_callback("click", function.as_ref().unchecked_ref())
    //         .unwrap();
    //     self.view
    //         .add_event_listener_with_callback("click", send.as_ref().unchecked_ref())
    //         .unwrap();

    //     self
    // }

    pub fn on_click(self, function: Box<dyn FnMut()>) -> Self {
        let function = Closure::wrap(function);

        self.view
            .add_event_listener_with_callback("click", function.as_ref().unchecked_ref())
            .unwrap();
        function.forget();

        self
    }
}
