use crate::tree::{RcTreeNode, Tree};
use crate::Component;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
pub use button::*;
pub use div::*;
pub use paragraph::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};
use vec_tree::VecTree;

pub mod button;
pub mod div;
pub mod paragraph;

#[derive(Clone)]
pub struct Html {
    content: String,
    tree: Tree,
}

impl Html {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            tree: Tree::default(),
        }
    }

    pub fn push(mut self, components: &[(String, RcTreeNode)]) -> Self {
        components.iter().for_each(|(x, y)| {
            self.content.push_str(&x);
            self.tree.insert(Rc::clone(y));
        });

        self
    }

    pub fn push_loop<F>(mut self, func: F, n: usize) -> Self
    where
        F: Fn(usize) -> (String, RcTreeNode),
    {
        (0..n).for_each(|x| {
            let (x, y) = func(x);
            self.content.push_str(&x);
            self.tree.insert(y);
        });

        self
    }

    pub fn view(&self) -> Element {
        let template = document()
            .create_element("div")
            .expect("Cannot create `template`");
        template.set_inner_html(&self.content);

        // let node = template.clone_node().unwrap();

        Self::link(&template, self.tree.root());

        template
    }

    fn link(node: &Node, tree_node: RcTreeNode) {
        let value = tree_node.borrow().value();

        if let Some(x) = value.borrow_mut().on_click() {
            let x = Closure::wrap(x);
            node.add_event_listener_with_callback("click", x.as_ref().unchecked_ref())
                .unwrap();
            x.forget()
        }

        let mut kids = node.first_child();
        let borrow = tree_node.borrow();
        let mut children = borrow.children().iter();

        while let (Some(kid), Some(child)) = (kids, children.next()) {
            Self::link(&kid, Rc::clone(child));

            kids = Some(kid);
            kids = kids.unwrap().next_sibling();
        }

        // for (i, child) in tree_node.borrow().children().iter().enumerate() {
        //     Self::link(children.item(i as u32).unwrap(), Rc::clone(child));
        // }
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

pub struct Function {
    on_click: Option<Box<dyn FnMut()>>,
}

impl Function {
    pub fn new() -> Self {
        Self { on_click: None }
    }

    pub fn on_click(&mut self) -> Option<Box<dyn FnMut()>> {
        self.on_click.take()
    }
}

impl Default for Function {
    fn default() -> Self {
        Self::new()
    }
}
