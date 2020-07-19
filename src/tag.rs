use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::Deref;
use core::str::FromStr;

use futures_intrusive::channel::StateId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::component;
use crate::function;
use crate::html;
use crate::state::StateTrait;
use crate::value::Value;

/// An HTML Tag
///
/// Macros are defined for easier use of few tags. Can also be used to make other tags also.
pub struct Tag<T> {
    elem_type: PhantomData<T>,
    pub(crate) node: crate::Node,
}

impl<T> Tag<T>
where
    T: html::elements::HtmlElement,
{
    /// Make a new `Tag`.
    /// You have to specify the Element type i.e. `Element` or `HtmlDivElement` etc.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// Tag::<html::elements::Div>::new().push("Hello, World!")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            elem_type: PhantomData,
            node: crate::Node::new(function::create_element(T::tag()).unchecked_into()),
        }
    }
}

impl<T> Tag<T> {
    /// Push components inside the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// div!().push("Hello, World!")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push<U>(self, component: U) -> Self
    where
        U: component::Component,
    {
        self.node.push_child(component.into());
        self
    }

    /// Push multiple components.
    /// Rarely used. Use `push` preferably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// div!().push_multiple(vec![Node::from("Hello, "), Node::from("World!")])
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push_multiple<U>(self, components: Vec<U>) -> Self
    where
        U: component::Component,
    {
        components.into_iter().fold(self, |tag, x| tag.push(x))
    }

    /// Push components as a loop from 0 to n (exclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// div!().push_loop(5, |x| h3!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push_loop<F, U>(self, n: usize, func: F) -> Self
    where
        F: Fn(usize) -> U,
        U: component::Component,
    {
        (0..n).map(func).fold(self, |tag, x| tag.push(x))
    }
}

impl<T> Tag<T>
where
    T: 'static,
{
    /// Attach an event to the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let message = StateMutex::new(String::from("App is running"));
    /// button!(message.clone())
    ///     .on_event("mouseover", message.clone(), |x, _| {
    ///         x.put("Mouse pointer is in me".to_string());
    ///     })
    ///     .on_event("mouseout", message.clone(), |x, _| {
    ///         x.put("Mouse pointer is outside".to_string());
    ///     })
    ///     .on_event("mousedown", message.clone(), |x, _| {
    ///         x.put("Mouse button pressed".to_string());
    ///     })
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn on_event<F, U>(self, event: impl AsRef<str>, mut var: U, mut func: F) -> Self
    where
        U: 'static,
        F: FnMut(&mut U, &mut Self) + 'static,
    {
        let mut tag = self.clone();
        let callback = Box::new(move || {
            func(&mut var, &mut tag);
        }) as Box<dyn FnMut()>;
        let x = Closure::wrap(callback);

        self.node.add_event_listener(event, x);

        self
    }
}

impl<T> Tag<T> {
    /// Remove an event from the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let message = StateMutex::new(String::from("App is running"));
    /// button!(message.clone())
    ///     .on_event("mouseover", message.clone(), |x, _| {
    ///         x.put("Mouse pointer is in me".to_string());
    ///     })
    ///     .on_event("mouseout", message.clone(), |x, _| {
    ///         x.put("Mouse pointer is outside".to_string());
    ///     })
    ///     .on_event("mousedown", message.clone(), |x, t| {
    ///         x.put("Mouse button pressed".to_string());
    ///         t.remove_event("mouseout");
    ///     })
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn remove_event(&self, event: impl AsRef<str>) {
        self.node.remove_event_listener(event);
    }

    /// Set the id of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// div!("Hello, World!")
    ///     .id("hello-world")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn id(self, id: impl AsRef<str>) -> Self {
        self.node.set_id(id);
        self
    }

    /// Get the id of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = h1!("Hello, World!").id("hello-world-id");
    /// div!(heading.clone(), br!(), "id ", heading.get_id().unwrap())
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn get_id(&self) -> Option<String> {
        self.node.get_id()
    }

    /// Set the class of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// div!("Hello, World!")
    ///     .class("heading")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn class(self, class: impl AsRef<str>) -> Self {
        self.node.insert_class(class);
        self
    }

    /// Get the classes of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = h1!("Hello, World!").class("heading");
    /// div!(heading.clone(), br!(), "class ", heading.get_class().join(" "))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn get_class(&self) -> Vec<String> {
        self.node.get_class()
    }

    /// Remove a class of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = input!("text").class("text-type");
    /// div!(
    ///     heading.clone(),
    ///     br!(),
    ///     button!("Remove class").on_event("click", heading, |x, _| {
    ///         x.rem_class("text-type");
    ///     })
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn rem_class(&self, class: impl AsRef<str>) {
        self.node.remove_class(class);
    }

    /// Toggle a class of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = input!("text").class("text-type");
    /// div!(
    ///     heading.clone(),
    ///     br!(),
    ///     button!("Toggle class").on_event("click", heading, |x, _| {
    ///         x.toggle_class("text-type");
    ///     })
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn toggle_class(&self, class: impl AsRef<str>) {
        self.node.toggle_class(class);
    }
}

impl<T> Tag<T>
where
    T: 'static,
{
    /// Set the attribute of the `Tag` by key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// div!("Hello, World!")
    ///     .attr("id", "hello-world")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn attr(self, key: impl AsRef<str>, value: impl Value) -> Self {
        let key = key.as_ref().to_string();
        let this = self.clone();
        value.bind_func(Box::new(move |x| this.node.set_attr(&key, x)));
        self
    }
}

impl<T> Tag<T> {
    /// Get the attribute of the `Tag` by key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = h1!("Hello, World!").attr("id", "hello-world");
    /// div!(
    ///     heading.clone(),
    ///     br!(),
    ///     "attr ",
    ///     heading.get_attr("id").unwrap()
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn get_attr(&self, key: impl AsRef<str>) -> Option<String> {
        self.node.get_attr(key)
    }

    /// Remove the attribute of the `Tag` by key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = input!("text").placeholder("Type Something");
    /// div!(
    ///     heading.clone(),
    ///     br!(),
    ///     button!("Remove placeholder").on_event("click", heading, |x, _| {
    ///         x.rem_attr("placeholder");
    ///     })
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn rem_attr(&self, key: impl AsRef<str>) {
        self.node.remove_attr(key);
    }
}

impl Tag<html::elements::Input> {
    /// One way bind to the input element.
    /// Any change in the state variable won't be reflected back to the input element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let state = StateMutex::new(String::new());
    /// div!(
    ///     state.clone(),
    ///     br!(),
    ///     input!("text").bind(state.clone()),
    ///     input!("text").bind(state)
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn bind<T>(self, var: T) -> Self
    where
        T: StateTrait + 'static,
        T::Value: FromStr + Default,
    {
        self.on_event("input", var, |x, elem| {
            x.put(
                elem.node
                    .node
                    .unchecked_ref::<web_sys::HtmlDataElement>()
                    .value()
                    .parse()
                    .unwrap_or_default(),
            );
        })
    }

    /// Two way bind to the input element.
    /// Any change in the state variable will be reflected back to the input element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let state = StateMutex::new(String::new());
    /// div!(
    ///     state.clone(),
    ///     br!(),
    ///     input!("text").double_bind(state.clone()),
    ///     input!("text").double_bind(state)
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn double_bind<T>(self, var: T) -> Self
    where
        T: StateTrait + 'static,
        T::Value: FromStr + Default,
        T::Channel: Deref<Target = String>,
    {
        let elem = self.node.clone();
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

    /// A function to bind to the input element.
    /// This function gets called every time the event `input` fires i.e. when the user enters and input.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let state = StateAtomic::new(0usize);
    /// div!(
    ///     state.clone(),
    ///     br!(),
    ///     input!("text").bind_func(state, |x| x.len())
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn bind_func<T, F>(self, var: T, func: F) -> Self
    where
        T: StateTrait + 'static,
        F: FnOnce(String) -> T::Value,
        F: 'static + Copy,
    {
        self.on_event("input", var, move |x, elem| {
            x.put(func(
                elem.node
                    .node
                    .unchecked_ref::<web_sys::HtmlDataElement>()
                    .value(),
            ));
        })
    }

    // /// To add placeholder for the input element.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # use valerie::prelude::*;
    // /// # use valerie::prelude::components::*;
    // /// # use wasm_bindgen_test::*;
    // /// # fn ui() -> Node {
    // /// input!("text")
    // ///     .placeholder("Enter something...")
    // /// # .into()
    // /// # }
    // /// # wasm_bindgen_test_configure!(run_in_browser);
    // /// # #[wasm_bindgen_test]
    // /// # fn run() {
    // /// #     App::render_single(ui());
    // /// # }
    // /// ```
    // pub fn placeholder(self, text: impl AsRef<str>) -> Self {
    //     self.node
    //         .unchecked_ref::<web_sys::HtmlInputElement>()
    //         .set_placeholder(text.as_ref());
    //
    //     self
    // }
}

impl<T> Clone for Tag<T> {
    fn clone(&self) -> Self {
        Self {
            elem_type: PhantomData,
            node: self.node.clone(),
        }
    }
}

impl<T> Default for Tag<T>
where
    T: html::elements::HtmlElement,
{
    fn default() -> Self {
        Self::new()
    }
}

// impl<T> Deref for Tag<T> {
//     type Target = crate::Node;
//
//     fn deref(&self) -> &Self::Target {
//         &self.node
//     }
// }
//
// impl<T> DerefMut for Tag<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.node
//     }
// }
//
// impl<T> AsRef<web_sys::Node> for Tag<T>
// where
//     T: JsCast,
// {
//     fn as_ref(&self) -> &web_sys::Node {
//         self.node.unchecked_ref()
//     }
// }
//
// impl<T> From<Tag<T>> for web_sys::Node
// where
//     T: JsCast,
// {
//     fn from(tag: Tag<T>) -> Self {
//         tag.node.unchecked_into()
//     }
// }

impl<T> From<Tag<T>> for crate::Node {
    fn from(tag: Tag<T>) -> Self {
        tag.node
    }
}

impl<T> component::Component for Tag<T> {}
