// use crate::Tree;
use crate::Channel;

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::string::String;
use futures_intrusive::channel::StateId;
use futures_intrusive::channel::shared::StateReceiver;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Text, Node};
use alloc::vec::Vec;
use core::ops::Deref;

pub struct Event {
    event: &'static str,
    callback: Box<dyn FnMut()>,
}

impl Event {
    pub fn new(event: &'static str, callback: Box<dyn FnMut()>) -> Self {
        Self {
            event,
            callback
        }
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
    // children: Option<Tree>,
    // pub rx: Option<Arc<Receiver>>,
    // pub events: Vec<Event>,
}

impl Function {
    pub fn new(function_type: FunctionType) -> Self {
        match function_type {
            FunctionType::Element(x) => Self {
                node: FunctionStore::Element(create_element(x)),
                // children: None,
                // rx: None,
                // events: Vec::new(),
            },
            FunctionType::Text(x) => Self {
                node: FunctionStore::Text(document().create_text_node(&x)),
                // children: None,
                // rx: None
                // events: Vec::new(),
            },
        }
        // Self {
        //     node: None,
        //     rx: None,
        //     events: Vec::new(),
        // }
    }

    pub fn push_child(&self, child: &Function) {
        if let FunctionStore::Element(x) = &self.node {
            x.append_child(match &child.node {
                FunctionStore::Element(x) => &x,
                FunctionStore::Text(x) => &x,
            }).unwrap();
        }
    } 

    pub fn add_event(&self, event: Event) {
        let Event { event , callback } = event;
        let x = Closure::wrap(callback);
        self.node().add_event_listener_with_callback(event, x.as_ref().unchecked_ref())
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
            _ => None
        }
    }

    // pub fn node(&mut self, node: Arc<Node>) {
    //     // self.node = Some(Arc::clone(&node));

    //     while let Some(Event {event, callback}) = self.events.pop() {
    //         let x = Closure::wrap(callback);
    //         node.add_event_listener_with_callback(event, x.as_ref().unchecked_ref())
    //             .unwrap();
    //         x.forget();
    //     }

    //     if self.rx.is_some() {
    //         wasm_bindgen_futures::spawn_local(change(Arc::clone(&node), self.rx.take().unwrap()));
    //     }
    // }
}

impl Deref for Function {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        self.node()
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
