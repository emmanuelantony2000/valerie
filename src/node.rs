use alloc::collections::{BTreeMap, BTreeSet};
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
    id: Arc<Mutex<Option<String>>>,
    class: Arc<Mutex<BTreeSet<String>>>,
    attr: Arc<Mutex<BTreeMap<String, String>>>,
    children: Arc<Mutex<Vec<Node>>>,
    callbacks: Arc<Mutex<CallbackMap>>,
}

impl Node {
    pub(crate) fn new(node: web_sys::Node) -> Self {
        Self {
            node,
            id: Arc::new(Mutex::new(None)),
            class: Arc::new(Mutex::new(BTreeSet::new())),
            attr: Arc::new(Mutex::new(BTreeMap::new())),
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

    pub(crate) fn set_id(&self, id: impl AsRef<str>) {
        let mut lock = self.id.lock();
        if let Some(x) = lock.as_ref() {
            if x == id.as_ref() {
                return;
            }
        }
        lock.replace(id.as_ref().to_string());
        self.node
            .unchecked_ref::<web_sys::Element>()
            .set_id(id.as_ref());
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.lock().clone()
    }

    pub(crate) fn insert_class(&self, class: impl AsRef<str>) {
        let mut lock = self.class.lock();
        if lock.insert(class.as_ref().to_string()) {
            self.set_class(
                lock.iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
    }

    pub(crate) fn remove_class(&self, class: impl AsRef<str>) {
        let mut lock = self.class.lock();
        if lock.remove(class.as_ref()) {
            self.set_class(
                lock.iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
    }

    pub(crate) fn get_class(&self) -> Vec<String> {
        self.class.lock().iter().map(String::clone).collect()
    }

    pub(crate) fn toggle_class(&self, class: impl AsRef<str>) {
        let mut lock = self.class.lock();
        if !lock.insert(class.as_ref().to_string()) {
            lock.remove(class.as_ref());
        }
        self.set_class(
            lock.iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(" "),
        );
    }

    pub(crate) fn set_attr(&self, attr: impl AsRef<str>, val: impl AsRef<str>) {
        let mut lock = self.attr.lock();
        if let Some(x) = lock.get(attr.as_ref()) {
            if x == val.as_ref() {
                return;
            }
        }
        lock.insert(attr.as_ref().to_string(), val.as_ref().to_string());
        self.node
            .unchecked_ref::<web_sys::Element>()
            .set_attribute(attr.as_ref(), val.as_ref())
            .unwrap();
    }

    pub(crate) fn remove_attr(&self, attr: impl AsRef<str>) {
        let mut lock = self.attr.lock();
        if lock.remove(attr.as_ref()).is_some() {
            self.node
                .unchecked_ref::<web_sys::Element>()
                .remove_attribute(attr.as_ref())
                .unwrap();
        }
    }

    pub(crate) fn get_attr(&self, attr: impl AsRef<str>) -> Option<String> {
        self.attr.lock().get(attr.as_ref()).cloned()
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

    fn set_class(&self, class: String) {
        self.node
            .unchecked_ref::<web_sys::Element>()
            .set_class_name(&class);
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
