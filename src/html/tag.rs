use crate::Component;
use crate::{Event, Function, FunctionType};

use alloc::boxed::Box;
use core::ops::Deref;
use web_sys::Node;

pub struct Tag {
    function: Function,
}

impl Tag {
    pub fn new(tag: &'static str) -> Self {
        Self {
            function: Function::new(FunctionType::Element(tag)),
        }
    }

    pub fn push<T>(self, component: &T) -> Self
    where
        T: Deref<Target = Node>,
    {
        self.push_child(component);

        self
    }

    pub fn push_multiple<T>(self, components: &[T]) -> Self
    where
        T: Deref<Target = Node>,
    {
        components.iter().for_each(|x| {
            self.push_child(x);
        });

        self
    }

    pub fn push_loop<F, T>(self, func: F, n: usize) -> Self
    where
        F: Fn(usize) -> T,
        T: Component,
    {
        (0..n).for_each(|x| {
            let x = func(x);
            self.push_child(&x.view());
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
        self.add_event(Event::new(
            event,
            Box::new(move || {
                func(&mut var);
            }),
        ));

        self
    }
}

impl Component for Tag {
    fn view(self) -> Function {
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
