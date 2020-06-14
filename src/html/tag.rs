use crate::Component;
use crate::{Event,State,Function, FunctionType};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Display;
use core::ops::Deref;

pub struct Tag {
    // tag: &'static str,
    // content: String,
    // tree: Option<Tree>,
    function: Function,
}

impl Tag {
    pub fn new(tag: &'static str) -> Self {
        Self {
            // tag,
            // content: String::new(),
            // tree: Some(Tree::new(Function::new())),
            function: Function::new(FunctionType::Element(tag)),
        }
    }

    // pub fn value<T>(&mut self, value: impl State<T>) -> &mut Self
    // where
    //     T: Display + Clone,
    // {
    //     let (x, mut y) = value.value().view();
    //     y.root_mut().data.rx = Some(value.rx());

    //     self.content.push_str(&x);
    //     self.tree.as_mut().unwrap().root_mut().push_back(y);

    //     self
    // }

    // pub fn push(&mut self, components: Vec<(String, Tree)>) -> &mut Self {
    //     components.into_iter().for_each(|(x, y)| {
    //         self.content.push_str(&x);
    //         self.tree.as_mut().unwrap().root_mut().push_back(y);
    //     });

    //     self
    // }

    pub fn push(self, components: &[Function]) -> Self {
        components.iter().for_each(|x| {
            self.push_child(x);
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
            self.push_child(&x);
        });

        self
    }

    pub fn on_click<T, F>(self, var: T, func: F) -> Self
    where
        T: 'static,
        F: FnMut(&mut T) + 'static,
    {
        self.on_event("click", var, func)
    }

    pub fn on_dblclick<T, F>(self, var: T, func: F) -> Self
    where
        T: 'static,
        F: FnMut(&mut T) + 'static,
    {
        self.on_event("dblclick", var, func)
    }

    pub fn on_event<T, F>(self, event: &'static str, mut var: T, mut func: F) -> Self
    where
        T: 'static,
        F: FnMut(&mut T) + 'static,
    {
        self.add_event(Event::new(event, Box::new(move || {
            func(&mut var);
        })));
        
        self
    }

    // pub fn on_click<T, U, F>(&mut self, state: &T, mut func: F) -> &mut Self
    // where
    //     T: State<U> + 'static,
    //     U: Display + Clone + 'static,
    //     F: FnMut(&mut T) + 'static,
    // {
    //     let mut state = state.clone();
    //     self.tree.as_mut().unwrap().root_mut().data.events.push(Event::new("click", Box::new(move || {
    //         func(&mut state);
    //         state.update();
    //     })));
        
    //     self
    // }
}

impl Component for Tag {
    fn view(self) -> Function {
        // let mut val = String::with_capacity((self.tag.len() * 2) + 5 + self.content.len());
        // val.push_str("<");
        // val.push_str(self.tag);
        // val.push_str(">");
        // val.push_str(&self.content);
        // val.push_str("</");
        // val.push_str(self.tag);
        // val.push_str(">");

        // (val, self.tree.take().unwrap())
        self.function
    }
}

impl Deref for Tag {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag::new("div")
    }
}