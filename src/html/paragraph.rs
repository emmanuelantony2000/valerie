use crate::Component;
use core::ops::Deref;
use web_sys::{Element, Node};
use core::fmt::Display;
use alloc::string::ToString;

#[derive(Clone)]
pub struct Paragraph {
    view: Element,
}

impl Component for Paragraph {
    fn view(&mut self) -> Element {
        self.view.clone()
    }
}

impl Deref for Paragraph {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl From<Paragraph> for Element {
    fn from(x: Paragraph) -> Self {
        x.view
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Paragraph {
    pub fn new() -> Self {
        Self {
            view: { document().create_element("p").expect("Cannot create `p`") },
        }
    }

    pub fn push_single(self, other: Element) -> Self {
        self.view.append_child(&other).unwrap();
        self
    }

    pub fn push(self, others: &[Element]) -> Self {
        others.iter().for_each(|i| {
            self.append_child(i).unwrap();
        });

        self
    }

    pub fn value<T>(self, content: T) -> Self where T: Display {
        self.view.set_inner_html(content.to_string().as_ref());
        self
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
