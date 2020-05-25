use super::Function;
use crate::Component;
use crate::Tree;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use futures_intrusive::channel::shared::StateReceiver;

pub struct Button {
    content: String,
    tree: Option<Tree>,
}

impl Button {
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

    pub fn push(mut self, components: Vec<(String, Tree)>) -> Self {
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

impl Component for Button {
    fn view(&mut self) -> (String, Tree) {
        let mut val = String::with_capacity(("<button>".len() * 2) + 1 + self.content.len());
        val.push_str("<button>");
        val.push_str(&self.content);
        val.push_str("</button>");

        (val, self.tree.take().unwrap())
    }
}

impl Default for Button {
    fn default() -> Self {
        Button::new()
    }
}
