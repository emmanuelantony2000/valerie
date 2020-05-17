use crate::Component;
use core::ops::Deref;
use web_sys::{Element, Node};

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

impl Paragraph {
    pub fn new() -> Self {
        Self {
            view: {
                web_sys::window()
                    .expect("No global `window` exits")
                    .document()
                    .expect("Should have a document on window")
                    .create_element("p")
                    .expect("Cannot create `p`")
            },
        }
    }

    pub fn push_single(self, other: Element) -> Self {
        self.view.append_child(&other).unwrap();
        self
    }

    pub fn push(self, others: &[Element]) -> Self {
        for i in others {
            self.append_child(i).unwrap();
        }

        self
    }

    pub fn value(self, content: &str) -> Self {
        self.view.set_inner_html(content);
        self
    }
}
