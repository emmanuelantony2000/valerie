use super::Function;
// use crate::tree::{RcTreeNode, Tree};
use trees::Tree;
use crate::Component;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, AtomicBool};
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use futures_intrusive::channel::shared::StateReceiver;

pub struct Paragraph {
    content: String,
    tree: Option<Tree<Function>>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            tree: Some(Tree::new(Function::new())),
        }
    }

    pub fn value(mut self, value: Arc<AtomicU64>, rx: StateReceiver<()>) -> Self {
        let (x, mut y) = value.load(Ordering::SeqCst).view();
        y.root_mut().data.value = Some(value);
        y.root_mut().data.rx = Some(rx);

        self.content.push_str(&x);
        self.tree.as_mut().unwrap().root_mut().push_back(y);

        self
    }

    pub fn push(mut self, components: Vec<(String, Tree<Function>)>) -> Self {
        components.into_iter().for_each(|(x, y)| {
            self.content.push_str(&x);
            self.tree.as_mut().unwrap().root_mut().push_back(y);
        });

        self
    }

    pub fn on_click(mut self, function: Box<dyn FnMut()>) -> Self {
        self.tree.as_mut().unwrap().root_mut().data.on_click = Some(function);
        self
    }
}

impl Component for Paragraph {
    fn view(&mut self) -> (String, Tree<Function>) {
        let mut val = String::with_capacity(("<p>".len() * 2) + 1 + self.content.len());
        val.push_str("<p>");
        val.push_str(&self.content);
        val.push_str("</p>");

        (val, self.tree.take().unwrap())
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Paragraph::new()
    }
}

// use crate::Component;
// use core::ops::Deref;
// use web_sys::{Element, Node};
// use core::fmt::Display;
// use alloc::string::ToString;

// #[derive(Clone)]
// pub struct Paragraph {
//     view: Element,
// }

// impl Component for Paragraph {
//     fn view(&mut self) -> Element {
//         self.view.clone()
//     }
// }

// impl Deref for Paragraph {
//     type Target = Node;

//     fn deref(&self) -> &Self::Target {
//         &self.view
//     }
// }

// impl From<Paragraph> for Element {
//     fn from(x: Paragraph) -> Self {
//         x.view
//     }
// }

// impl Default for Paragraph {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Paragraph {
//     pub fn new() -> Self {
//         Self {
//             view: { document().create_element("p").expect("Cannot create `p`") },
//         }
//     }

//     pub fn push_single(self, other: Element) -> Self {
//         self.view.append_child(&other).unwrap();
//         self
//     }

//     pub fn push(self, others: &[Element]) -> Self {
//         others.iter().for_each(|i| {
//             self.append_child(i).unwrap();
//         });

//         self
//     }

//     pub fn value<T>(self, content: T) -> Self where T: Display {
//         self.view.set_inner_html(content.to_string().as_ref());
//         self
//     }
// }

// fn window() -> web_sys::Window {
//     web_sys::window().expect("No global `window` exists")
// }

// fn document() -> web_sys::Document {
//     window()
//         .document()
//         .expect("Should have a document on window")
// }
