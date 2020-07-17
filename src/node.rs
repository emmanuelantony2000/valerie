use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use parking_lot::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::component;

type CallbackMap = BTreeMap<String, Rc<Closure<dyn FnMut()>>>;

/// A wrapper for `web_sys::Node`
#[derive(Clone)]
pub struct Node {
    pub(crate) node: web_sys::Node,
    children: Arc<Mutex<Vec<Node>>>,
    callbacks: Arc<Mutex<CallbackMap>>,
}

impl Node {
    pub(crate) fn new(node: web_sys::Node) -> Self {
        Self {
            node,
            children: Arc::new(Mutex::new(Vec::new())),
            callbacks: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub(crate) fn push_child(&self, node: Self) {
        self.node.append_child(node.as_ref()).unwrap();
        self.children.lock().push(node);
    }

    pub(crate) fn insert_child(&self, index: usize, node: Self) {
        let mut lock = self.children.lock();
        lock.insert(index, node);
        self.node
            .insert_before(lock[index].as_ref(), Some(lock[index + 1].as_ref()))
            .unwrap();
    }

    pub(crate) fn pop_child(&self) {
        if let Some(node) = self.children.lock().pop() {
            self.node.remove_child(node.as_ref()).unwrap();
        }
    }

    pub(crate) fn remove_child(&self, index: usize) {
        let mut lock = self.children.lock();
        if lock.get(index).is_some() {
            self.node.remove_child(lock.remove(index).as_ref()).unwrap();
        }
    }

    pub(crate) fn add_event_listener(&self, event: impl AsRef<str>, x: Closure<dyn FnMut()>) {
        let x = Rc::new(x);
        self.node
            .add_event_listener_with_callback(event.as_ref(), x.as_ref().as_ref().unchecked_ref())
            .unwrap();
        self.callbacks.lock().insert(event.as_ref().to_string(), x);
    }

    pub(crate) fn remove_event_listener(&self, event: impl AsRef<str>) {
        if let Some(x) = self.callbacks.lock().remove(event.as_ref()) {
            self.node
                .remove_event_listener_with_callback(
                    event.as_ref(),
                    x.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();
        }
    }

    pub(crate) fn forget(self) {
        core::mem::forget(self);
    }
}

impl Deref for Node {
    type Target = web_sys::Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl From<Node> for web_sys::Node {
    fn from(x: Node) -> Self {
        x.node
    }
}

impl AsRef<web_sys::Node> for Node {
    fn as_ref(&self) -> &web_sys::Node {
        &self.node
    }
}

impl component::Component for Node {}
