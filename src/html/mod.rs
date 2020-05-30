use crate::Function;
use crate::Tree;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::pin::Pin;
use web_sys::{Element, Node};

pub use button::*;
pub use div::*;
pub use paragraph::*;
pub use tag::*;

pub mod button;
pub mod div;
pub mod paragraph;
pub mod tag;

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

        self.content = String::new();

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
