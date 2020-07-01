use alloc::boxed::Box;
use alloc::string::String;
use core::ops::{Deref, DerefMut};
use core::str::FromStr;

use futures_intrusive::channel::StateId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::component;
use crate::function;
use crate::state::StateTrait;

pub struct Tag<T> {
    elem: T,
}

impl<T> Tag<T>
where
    T: wasm_bindgen::JsCast,
{
    pub fn new(tag: &'static str) -> Self {
        Self {
            elem: function::create_element(tag).unchecked_into(),
        }
    }
}

impl<T> Tag<T>
where
    T: AsRef<web_sys::Node>,
{
    pub fn push<U>(self, component: &U) -> Self
    where
        U: AsRef<web_sys::Node>,
    {
        self.elem.as_ref().append_child(component.as_ref()).unwrap();

        self
    }

    pub fn push_multiple<U>(self, components: &[U]) -> Self
    where
        U: AsRef<web_sys::Node>,
    {
        components.iter().for_each(|x| {
            self.elem.as_ref().append_child(x.as_ref()).unwrap();
        });

        self
    }

    pub fn push_loop<F, U>(self, func: F, n: usize) -> Self
    where
        F: Fn(usize) -> T,
        T: component::Component,
    {
        (0..n).for_each(|x| {
            let x = func(x);
            self.elem.as_ref().append_child(x.view().as_ref()).unwrap();
        });

        self
    }
}

impl<T> Tag<T>
where
    T: AsRef<web_sys::Node> + Clone + 'static,
{
    pub fn on_event<F, U>(self, event: impl AsRef<str>, mut var: U, mut func: F) -> Self
    where
        U: 'static,
        F: FnMut(&mut U, &mut T) + 'static,
    {
        let mut elem = self.elem.clone();
        let callback = Box::new(move || {
            func(&mut var, &mut elem);
        }) as Box<dyn FnMut()>;
        let x = Closure::wrap(callback);
        self.elem
            .as_ref()
            .add_event_listener_with_callback(event.as_ref(), x.as_ref().unchecked_ref())
            .unwrap();
        x.forget();

        self
    }
}

impl<T> Tag<T>
where
    T: AsRef<web_sys::Element>,
{
    pub fn id(self, id: impl AsRef<str>) -> Self {
        self.as_ref().set_id(id.as_ref());

        self
    }

    pub fn get_id(&self) -> String {
        self.as_ref().id()
    }

    pub fn class(self, class: impl AsRef<str>) -> Self {
        self.as_ref().set_class_name(class.as_ref());

        self
    }

    pub fn get_class(&self) -> String {
        self.as_ref().class_name()
    }

    pub fn attr(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.as_ref()
            .set_attribute(key.as_ref(), value.as_ref())
            .unwrap();

        self
    }

    pub fn get_attr(&self, key: impl AsRef<str>) -> Option<String> {
        self.as_ref().get_attribute(key.as_ref())
    }
}

impl Tag<web_sys::HtmlInputElement> {
    pub fn bind<T>(self, var: T) -> Self
    where
        T: StateTrait + 'static,
        T::Value: FromStr + Default,
    {
        self.on_event("input", var, |x, elem| {
            x.put(
                elem.unchecked_ref::<web_sys::HtmlDataElement>()
                    .value()
                    .parse()
                    .unwrap_or_default(),
            );
        })
    }

    pub fn double_bind<T>(self, var: T) -> Self
    where
        T: StateTrait + 'static,
        T::Value: FromStr + Default,
        T::Channel: Deref<Target = String>,
    {
        let elem = self.elem.clone();
        let rx = var.rx();

        wasm_bindgen_futures::spawn_local(async move {
            let mut old = StateId::new();
            while let Some((new, value)) = rx.receive(old).await {
                elem.unchecked_ref::<web_sys::HtmlDataElement>()
                    .set_value(&value);
                old = new;
            }
        });

        self.bind(var)
    }

    pub fn bind_func<T, F>(self, var: T, func: F) -> Self
    where
        T: StateTrait + 'static,
        F: FnOnce(String) -> T::Value,
        F: 'static + Copy,
    {
        self.on_event("input", var, move |x, elem| {
            x.put(func(
                elem.unchecked_ref::<web_sys::HtmlDataElement>().value(),
            ));
        })
    }

    pub fn placeholder(self, text: impl AsRef<str>) -> Self {
        self.elem.set_placeholder(text.as_ref());

        self
    }
}

impl<T> Deref for Tag<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl<T> DerefMut for Tag<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elem
    }
}

impl<T> AsRef<web_sys::Node> for Tag<T>
where
    T: AsRef<web_sys::Node>,
{
    fn as_ref(&self) -> &web_sys::Node {
        self.elem.as_ref()
    }
}

impl<T> From<Tag<T>> for web_sys::Node
where
    T: JsCast,
{
    fn from(tag: Tag<T>) -> Self {
        tag.elem.unchecked_into()
    }
}

impl<T> component::Component for Tag<T>
where
    T: JsCast,
{
    type Type = web_sys::Node;

    fn view(self) -> Self::Type {
        self.elem.unchecked_into()
    }
}

// pub struct Tag {
//     function: Function,
// }

// impl Tag {
//     pub fn new(tag: &'static str) -> Self {
//         Self {
//             function: Function::new(FunctionType::Element(tag)),
//         }
//     }

//     pub fn push<T>(self, component: &T) -> Self
//     where
//         T: Deref<Target = web_sys::Node>,
//     {
//         self.push_child(component);

//         self
//     }

//     pub fn push_multiple<T>(self, components: &[T]) -> Self
//     where
//         T: Deref<Target = web_sys::Node>,
//     {
//         components.iter().for_each(|x| {
//             self.push_child(x);
//         });

//         self
//     }

//     pub fn push_loop<F, T>(self, func: F, n: usize) -> Self
//     where
//         F: Fn(usize) -> T,
//         T: Component,
//     {
//         (0..n).for_each(|x| {
//             let x = func(x);
//             self.push_child(&x.view());
//         });

//         self
//     }

//     pub fn id(self, id: &'static str) -> Self {
//         self.set_id(id);

//         self
//     }

//     pub fn get_id(&self) -> String {
//         self.deref().get_id().unwrap()
//     }

//     pub fn class(self, class: &'static str) -> Self {
//         self.set_class(class);

//         self
//     }

//     pub fn get_class(&self) -> String {
//         self.deref().get_class().unwrap()
//     }

//     pub fn attr(self, attr: &'static str, value: &'static str) -> Self {
//         self.set_attribute(attr, value);

//         self
//     }

//     pub fn get_attr(&self, attr: &'static str) -> Option<String> {
//         self.get_attribute(attr)
//     }

//     pub fn on_click<T, F>(self, var: T, func: F) -> Self
//     where
//         T: 'static,
//         F: FnMut(&mut T, &mut Element) + 'static,
//     {
//         self.on_event("click", var, func)
//     }

//     pub fn on_dblclick<T, F>(self, var: T, func: F) -> Self
//     where
//         T: 'static,
//         F: FnMut(&mut T, &mut Element) + 'static,
//     {
//         self.on_event("dblclick", var, func)
//     }

//     pub fn on_event<T, F>(self, event: &'static str, mut var: T, mut func: F) -> Self
//     where
//         T: 'static,
//         F: FnMut(&mut T, &mut Element) + 'static,
//     {
//         let mut elem = self.element().unwrap();
//         self.add_event(Event::new(
//             event,
//             Box::new(move || {
//                 func(&mut var, &mut elem);
//             }),
//         ));

//         self
//     }

//     pub fn bind<T>(self, var: T) -> Self
//     where
//         T: StateTrait + 'static,
//         T::Value: FromStr + Default,
//     {
//         self.on_event("input", var, |x, elem| {
//             x.put(
//                 elem.unchecked_ref::<web_sys::HtmlDataElement>()
//                     .value()
//                     .parse()
//                     .unwrap_or_default(),
//             );
//         })
//     }

//     pub fn double_bind<T>(self, var: T) -> Self
//     where
//         T: StateTrait + 'static,
//         T::Value: FromStr + Default,
//         T::Channel: Deref<Target = String>,
//     {
//         let ele = self.element().unwrap();
//         let rx = var.rx();

//         wasm_bindgen_futures::spawn_local(async move {
//             let mut old = StateId::new();
//             while let Some((new, value)) = rx.receive(old).await {
//                 ele.unchecked_ref::<web_sys::HtmlDataElement>().set_value(&value);
//                 old = new;
//             }
//         });

//         self.bind(var)
//     }

//     pub fn bind_func<T, F>(self, var: T, func: F) -> Self
//     where
//         T: StateTrait + 'static,
//         F: FnOnce(String) -> T::Value,
//         F: 'static + Copy,
//     {
//         self.on_event("input", var, move |x, elem| {
//             x.put(func(elem.unchecked_ref::<web_sys::HtmlDataElement>().value()));
//         })
//     }
// }

// impl Component for Tag {
//     fn view(self) -> Function {
//         self.function
//     }
// }

// impl Deref for Tag {
//     type Target = Function;

//     fn deref(&self) -> &Self::Target {
//         &self.function
//     }
// }

// impl Default for Tag {
//     fn default() -> Self {
//         Tag::new("div")
//     }
// }
