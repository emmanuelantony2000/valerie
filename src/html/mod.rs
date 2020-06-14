use crate::Function;
use crate::FunctionType;
// use crate::Tree;
use crate::Component;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::pin::Pin;
use web_sys::{Element, Node};

// pub use button::*;
// pub use div::*;
// pub use paragraph::*;
pub use tag::*;

// pub mod button;
// pub mod div;
// pub mod paragraph;
pub mod tag;

pub struct Html {
    // content: String,
    root: Function,
}

impl Html {
    pub fn new() -> Self {
        Self {
            // content: String::new(),
            // tree: Tree::new(Function::new()),
            root: Tag::new("div").view(),
        }
    }

    pub fn push(self, components: &[Function]) -> Self {
        components.iter().for_each(|x| {
            // self.content.push_str(&x);
            // self.tree.root_mut().push_back(y);
            self.root.push_child(x)
        });

        self
    }

    pub fn push_loop<F>(self, func: F, n: usize) -> Self
    where
        F: Fn(usize) -> Function,
    {
        (0..n).for_each(|x| {
            let x = func(x);
            // self.content.push_str(&x);
            // self.tree.root_mut().push_back(y);
            self.root.push_child(&x);
        });

        self
    }

    pub fn view(&mut self) -> Element {
        let template = create_element("div");
        // template.set_inner_html(&self.content);
        template.append_child(&self.root).unwrap();

        // let child = template.first_child();
        // if let Some(x) = child {
        //     let arc: Arc<Node> = Arc::new(x.parent_node().unwrap());
        //     Self::link(Arc::clone(&arc), self.tree.root_mut());
        // }

        // self.content = String::new();

        template
    }

    // fn link(node: Arc<Node>, mut tree_node: Pin<&mut trees::Node<Function>>) {
    //     tree_node.data.node(Arc::clone(&node));

    //     let mut kids = node.first_child();

    //     for child in tree_node.iter_mut() {
    //         let arc = Arc::new(kids.unwrap());
    //         Self::link(Arc::clone(&arc), child);
    //         kids = arc.next_sibling();
    //     }
    // }
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

fn create_element(name: &'static str) -> Element {
    document()
        .create_element(name)
        .expect("Cannot create the element needed")
}

pub fn div(components: &[Function]) -> Tag {
    Tag::new("div").push(components)
}

pub fn p(components: &[Function]) -> Tag {
    Tag::new("p").push(components)
}

pub fn button(components: &[Function]) -> Tag {
    Tag::new("button").push(components)
}

pub fn h1(components: &[Function]) -> Tag {
    Tag::new("h1").push(components)
}

pub fn h2(components: &[Function]) -> Tag {
    Tag::new("h2").push(components)
}

pub fn h3(components: &[Function]) -> Tag {
    Tag::new("h3").push(components)
}

pub fn h4(components: &[Function]) -> Tag {
    Tag::new("h4").push(components)
}

pub fn h5(components: &[Function]) -> Tag {
    Tag::new("h5").push(components)
}

pub fn h6(components: &[Function]) -> Tag {
    Tag::new("h6").push(components)
}

pub fn br() -> Tag {
    Tag::new("br")
}

pub fn span(components: &[Function]) -> Tag {
    Tag::new("span").push(components)
}

pub fn ul(components: &[Function]) -> Tag {
    Tag::new("ul").push(components)
}

pub fn ol(components: &[Function]) -> Tag {
    Tag::new("ol").push(components)
}

pub fn li(components: &[Function]) -> Tag {
    Tag::new("li").push(components)
}
