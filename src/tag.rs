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

/// An HTML Tag
///
/// Macros are defined for easier use of few tags. Can also be used to make other tags also.
#[derive(Clone)]
pub struct Tag<T> {
    elem: T,
}

impl<T> Tag<T>
where
    T: wasm_bindgen::JsCast,
    T: AsRef<web_sys::Element>,
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
    /// Tag::<web_sys::Element>::new("div").push("Hello, World!")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
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
        self.elem
            .as_ref()
            .append_child(component.into().as_ref())
            .unwrap();

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
    /// div!().push_multiple(&[Node::from("Hello, "), Node::from("World!")])
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push_multiple<U>(self, components: &[U]) -> Self
    where
        U: AsRef<web_sys::Node>,
    {
        components.iter().for_each(|x| {
            self.elem.as_ref().append_child(x.as_ref()).unwrap();
        });

        self
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
    T: AsRef<web_sys::Node> + Clone + 'static,
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
    /// let message = StateMutex::new(String::new());
    /// button!(message.clone())
    ///     .on_event("mouseover", message.clone(), |x, _| {
    ///         x.put("Mouse pointer is in me".to_string())
    ///     })
    ///     .on_event("mouseout", message.clone(), |x, _| {
    ///         x.put("Mouse pointer is outside".to_string())
    ///     })
    ///     .on_event("mousedown", message.clone(), |x, _| {
    ///         x.put("Mouse button pressed".to_string())
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
        self.as_ref().set_id(id.as_ref());

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
    /// div!(heading.clone(), br!(), "id ", heading.get_id())
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn get_id(&self) -> String {
        self.as_ref().id()
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
        self.as_ref().set_class_name(class.as_ref());

        self
    }

    /// Get the class of the `Tag`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let heading = h1!("Hello, World!").class("heading");
    /// div!(heading.clone(), br!(), "class ", heading.get_class())
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn get_class(&self) -> String {
        self.as_ref().class_name()
    }

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
    pub fn attr(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.as_ref()
            .set_attribute(key.as_ref(), value.as_ref())
            .unwrap();

        self
    }

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
        self.as_ref().get_attribute(key.as_ref())
    }
}

impl Tag<web_sys::HtmlInputElement> {
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
                elem.unchecked_ref::<web_sys::HtmlDataElement>()
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
                elem.unchecked_ref::<web_sys::HtmlDataElement>().value(),
            ));
        })
    }

    /// To add placeholder for the input element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// input!("text")
    ///     .placeholder("Enter something...")
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
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
    T: JsCast,
{
    fn as_ref(&self) -> &web_sys::Node {
        self.elem.unchecked_ref()
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

impl<T> From<Tag<T>> for crate::Node
where
    T: JsCast,
{
    fn from(tag: Tag<T>) -> Self {
        Self(tag.elem.unchecked_into())
    }
}

impl<T> component::Component for Tag<T> where T: JsCast {}
