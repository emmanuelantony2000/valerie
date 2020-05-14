use crate::Component;
use core::ops::Deref;
use web_sys::{Element, Node};

pub struct Div {
    view: Box<Element>,
}

impl Component for Div {}

impl Deref for Div {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl Div {
    pub fn view() -> Self {
        Self {
            view: {
                Box::new(
                    web_sys::window()
                        .expect("No global `window` exits")
                        .document()
                        .expect("Should have a document on window")
                        .create_element("div")
                        .expect("Cannot create `div`"),
                )
            },
        }
    }

    pub fn push_single(self, other: Box<dyn Deref<Target = Node>>) -> Self {
        self.view.append_child(&other).unwrap();
        self
    }

    pub fn push(mut self, others: Vec<Box<dyn Deref<Target = Node>>>) -> Self {
        for i in others {
            self = self.push_single(i);
        }

        self
    }

    pub fn value(self, content: &str) -> Self {
        self.view.set_inner_html(content);
        self
    }
}
