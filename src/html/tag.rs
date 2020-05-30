use crate::Component;
use crate::State;
use crate::Tree;
use crate::Function;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Display;

pub struct Tag {
    tag: &'static str,
    content: String,
    tree: Option<Tree>,
}

impl Tag {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            content: String::new(),
            tree: Some(Tree::new(Function::new())),
        }
    }

    pub fn value<T>(&mut self, value: impl State<T>) -> &mut Self
    where
        T: Display + Clone,
    {
        let (x, mut y) = value.value().view();
        y.root_mut().data.rx = Some(value.rx());

        self.content.push_str(&x);
        self.tree.as_mut().unwrap().root_mut().push_back(y);

        self
    }

    pub fn push(&mut self, components: Vec<(String, Tree)>) -> &mut Self {
        components.into_iter().for_each(|(x, y)| {
            self.content.push_str(&x);
            self.tree.as_mut().unwrap().root_mut().push_back(y);
        });

        self
    }

    pub fn on_click<T, U, F>(&mut self, state: &T, mut func: F) -> &mut Self
    where
        T: State<U> + 'static,
        U: Display + Clone + 'static,
        F: FnMut(&mut T) + 'static,
    {
        let mut state = state.clone();
        self.tree.as_mut().unwrap().root_mut().data.on_click = Some(Box::new(move || {
            func(&mut state);
            state.update();
        }));
        
        self
    }
}

impl Component for Tag {
    fn view(&mut self) -> (String, Tree) {
        let mut val = String::with_capacity((self.tag.len() * 2) + 5 + self.content.len());
        val.push_str("<");
        val.push_str(self.tag);
        val.push_str(">");
        val.push_str(&self.content);
        val.push_str("</");
        val.push_str(self.tag);
        val.push_str(">");

        (val, self.tree.take().unwrap())
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag::new("div")
    }
}