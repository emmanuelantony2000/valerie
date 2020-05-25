use crate::Tree;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::pin::Pin;
use core::sync::atomic::{AtomicU64, Ordering};
use futures_intrusive::channel::shared::StateReceiver;
use futures_intrusive::channel::StateId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

pub use button::*;
pub use div::*;
pub use paragraph::*;

pub mod button;
pub mod div;
pub mod paragraph;

pub struct Html {
    content: String,
    tree: Tree,
}

impl Html {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            tree: Tree::new(Function::new()),
        }
    }

    pub fn push(mut self, components: Vec<(String, Tree)>) -> Self {
        components.into_iter().for_each(|(x, y)| {
            self.content.push_str(&x);
            self.tree.root_mut().push_back(y);
        });

        self
    }

    pub fn push_loop<F>(mut self, func: F, n: usize) -> Self
    where
        F: Fn(usize) -> (String, Tree),
    {
        (0..n).for_each(|x| {
            let (x, y) = func(x);
            self.content.push_str(&x);
            self.tree.root_mut().push_back(y);
        });

        self
    }

    pub fn view(&mut self) -> Element {
        let template = document()
            .create_element("div")
            .expect("Cannot create `template`");
        template.set_inner_html(&self.content);

        let child = template.first_child();
        if let Some(x) = child {
            let arc: Arc<Node> = Arc::new(x.parent_node().unwrap());
            Self::link(Arc::clone(&arc), self.tree.root_mut());
        }

        template
    }

    fn link(node: Arc<Node>, mut tree_node: Pin<&mut trees::Node<Function>>) {
        tree_node.data.node(Arc::clone(&node));

        let mut kids = node.first_child();

        for child in tree_node.iter_mut() {
            let arc = Arc::new(kids.unwrap());
            Self::link(Arc::clone(&arc), child);
            kids = arc.next_sibling();
        }
    }
}

impl Default for Html {
    fn default() -> Self {
        Self::new()
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

#[derive(Default)]
pub struct Function {
    node: Option<Arc<Node>>,
    value: Option<Arc<AtomicU64>>,
    rx: Option<StateReceiver<()>>,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            node: None,
            value: None,
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

        if let Some(x) = self.value.as_ref() {
            wasm_bindgen_futures::spawn_local(change(
                Arc::clone(&node),
                Arc::clone(&x),
                self.rx.take().unwrap(),
            ));
        }
    }

    pub fn on_click(&mut self) -> Option<Box<dyn FnMut()>> {
        self.on_click.take()
    }
}

pub async fn change(node: Arc<Node>, value: Arc<AtomicU64>, rx: StateReceiver<()>) {
    let mut old = StateId::new();
    while let Some((new, _)) = rx.receive(old).await {
        node.set_node_value(Some(&value.load(Ordering::SeqCst).to_string()));
        old = new;
    }
}
