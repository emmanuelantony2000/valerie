use crate::{Channel, Component};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use core::ops::Deref;
use futures_intrusive::channel::shared::StateReceiver;
use futures_intrusive::channel::StateId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node, Text};

pub struct Event {
    event: &'static str,
    callback: Box<dyn FnMut()>,
}

impl Event {
    pub fn new(event: &'static str, callback: Box<dyn FnMut()>) -> Self {
        Self { event, callback }
    }
}

pub enum FunctionType {
    Element(&'static str),
    Text(String),
}

#[derive(Clone)]
pub enum FunctionStore {
    Element(Element),
    Text(Text),
}

#[derive(Clone)]
pub struct Function {
    node: FunctionStore,
}

impl Function {
    pub fn new(function_type: FunctionType) -> Self {
        match function_type {
            FunctionType::Element(x) => Self {
                node: FunctionStore::Element(create_element(x)),
            },
            FunctionType::Text(x) => Self {
                node: FunctionStore::Text(document().create_text_node(&x)),
            },
        }
    }

    pub fn push_child<T>(&self, child: &T)
    where
        T: Deref<Target = Node>,
    {
        if let FunctionStore::Element(x) = &self.node {
            x.append_child(&child).unwrap();
        }
    }

    pub fn add_event(&self, event: Event) {
        let Event { event, callback } = event;
        let x = Closure::wrap(callback);
        self.node()
            .add_event_listener_with_callback(event, x.as_ref().unchecked_ref())
            .unwrap();
        x.forget();
    }

    pub fn node(&self) -> &Node {
        match &self.node {
            FunctionStore::Element(x) => &x,
            FunctionStore::Text(x) => &x,
        }
    }

    pub fn element(&self) -> Option<Element> {
        match &self.node {
            FunctionStore::Element(x) => Some(x.clone()),
            _ => None,
        }
    }
}

impl Deref for Function {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        self.node()
    }
}

impl Component for Function {
    fn view(self) -> Self {
        self
    }
}

pub async fn change(node: Arc<Node>, rx: Arc<StateReceiver<Channel>>) {
    let mut old = StateId::new();
    while let Some((new, value)) = rx.receive(old).await {
        node.set_node_value(Some(&value));
        old = new;
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

fn create_element(name: &'static str) -> Element {
    document()
        .create_element(name)
        .expect("Cannot create the element needed")
}
