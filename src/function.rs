use crate::Receiver;

use alloc::boxed::Box;
use alloc::sync::Arc;
use futures_intrusive::channel::StateId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use alloc::vec::Vec;

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

#[derive(Default)]
pub struct Function {
    node: Option<Arc<Node>>,
    pub rx: Option<Arc<Receiver>>,
    pub events: Vec<Event>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            node: None,
            rx: None,
            events: Vec::new(),
        }
    }

    pub fn node(&mut self, node: Arc<Node>) {
        self.node = Some(Arc::clone(&node));

        while let Some(Event {event, callback}) = self.events.pop() {
            let x = Closure::wrap(callback);
            node.add_event_listener_with_callback(event, x.as_ref().unchecked_ref())
                .unwrap();
            x.forget();
        }

        if self.rx.is_some() {
            wasm_bindgen_futures::spawn_local(change(Arc::clone(&node), self.rx.take().unwrap()));
        }
    }
}

pub async fn change(node: Arc<Node>, rx: Arc<Receiver>) {
    let mut old = StateId::new();
    while let Some((new, value)) = rx.receive(old).await {
        node.set_node_value(Some(&value));
        old = new;
    }
}
