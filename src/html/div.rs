use crate::Component;
use core::ops::Deref;
use web_sys::{Element, Node};

#[derive(Clone)]
pub struct Div {
    view: Element,
}

impl Component for Div {
    fn view(&mut self) -> Element {
        self.view.clone()
    }
}

impl Deref for Div {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl From<Div> for Element {
    fn from(x: Div) -> Self {
        x.view
    }
}

impl Default for Div {
    fn default() -> Self {
        Self::new()
    }
}

impl Div {
    pub fn new() -> Self {
        Self {
            view: {
                document()
                    .create_element("div")
                    .expect("Cannot create `div`")
            },
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

    pub fn push_loop<F>(self, func: F, n: usize) -> Self where F: Fn(usize)-> Element {
        (0..n).for_each(|i| {
            self.append_child(&func(i)).unwrap();
        });

        self
    }

    pub fn value(self, content: &str) -> Self {
        self.view.set_inner_html(content);
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
