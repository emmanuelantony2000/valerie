use crate::Receiver;

use alloc::boxed::Box;
use alloc::sync::Arc;
use futures_intrusive::channel::StateId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;

#[derive(Default)]
pub struct Function {
    node: Option<Arc<Node>>,
    pub rx: Option<Arc<Receiver>>,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            node: None,
            rx: None,
            on_click: None,
        }
    }

    pub fn node(&mut self, node: Arc<Node>) {
        self.node = Some(Arc::clone(&node));

        let on_click = self.on_click();
        if let Some(x) = on_click {
            let x = Closure::wrap(x);
            node.add_event_listener_with_callback("click", x.as_ref().unchecked_ref())
                .unwrap();
            x.forget();
        }

        if self.rx.is_some() {
            wasm_bindgen_futures::spawn_local(change(Arc::clone(&node), self.rx.take().unwrap()));
        }
    }

    pub fn on_click(&mut self) -> Option<Box<dyn FnMut()>> {
        self.on_click.take()
    }
}

pub async fn change(node: Arc<Node>, rx: Arc<Receiver>) {
    let mut old = StateId::new();
    while let Some((new, value)) = rx.receive(old).await {
        node.set_node_value(Some(&value));
        old = new;
    }
}
