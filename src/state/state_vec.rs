use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Display;
use core::iter::FromIterator;
// use core::ops::{Index, IndexMut};

use futures_intrusive::channel::shared::{unbuffered_channel, Sender};
use parking_lot::RwLock;

use super::{StateAtomic, StateMutex, StateTrait};
use crate::component;
use crate::html;

#[derive(Clone)]
enum Change<T>
where
    T: Send,
{
    Insert(usize, T),
    Push(T),
    Remove(usize),
    Pop,
}

/// A vector of States
///
/// Any type implementing `StateTrait` can be used with StateVec.
///
/// This uses `RwLock` of parking_lot internally with `Vec` from alloc.
#[derive(Default)]
pub struct StateVec<T>
where
    T: Send + 'static,
{
    value: Arc<RwLock<Vec<T>>>,
    tx: Arc<RwLock<Vec<Sender<Change<T>>>>>,
}

impl<T> StateVec<T>
where
    T: Send,
{
    /// Declare an empty StateVec.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.push(StateAtomic::new(0));
    ///
    /// vec.view(ul!(), |x| li!(x))
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
            value: Arc::new(RwLock::new(Vec::new())),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Declare a StateVec with some initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn with_capacity(n: usize) -> Self {
        Self {
            value: Arc::new(RwLock::new(Vec::with_capacity(n))),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<T> StateVec<T>
where
    T: StateTrait + Send,
{
    /// Render the StateVec to the DOM.
    /// It will update if any element changes or even if the list changes automatically.
    ///
    ///  - `enclose` is the `Component` inside which all of the elements will be present.
    ///  - `object` is the function which will return a `Component` when passed a type following
    /// the `StateTrait`. The function will define how all the elements will be seen inside
    /// the `enclose`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    ///
    /// div!(
    ///     vec.view(ul!(), |x| li!(x)),
    ///     br!(),
    ///     vec.view(ol!(), |x| li!("Element ", x))
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn view<F, U, V>(&self, enclose: crate::Tag<U>, object: F) -> crate::Tag<U>
    where
        F: FnOnce(T) -> V,
        F: Clone + 'static,
        V: component::Component + 'static,
        U: html::elements::HtmlElement,
    {
        // let mut nodes = Vec::with_capacity(self.len());
        for i in self.value.read().iter() {
            // nodes.push(object.clone()(i.clone()));
            // enclose
            //     .as_ref()
            //     .append_child(nodes.last().unwrap().as_ref())
            //     .unwrap();
            enclose.node.push_child(object.clone()(i.clone()).into())
        }

        // let element = enclose.as_ref().clone();
        let node = enclose.node.clone();
        let (tx, rx) = unbuffered_channel();
        self.tx.write().push(tx);
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(change) = rx.receive().await {
                match change {
                    Change::Insert(i, x) => node.insert_child(i, object.clone()(x).into()),
                    // {
                    // nodes.insert(i, object.clone()(x));
                    // element
                    //     .insert_before(&nodes[i].as_ref(), Some(&nodes[i + 1].as_ref()))
                    //     .unwrap();
                    // }
                    Change::Push(x) => node.push_child(object.clone()(x).into()),
                    //     {
                    //     nodes.push(object.clone()(x));
                    //     element
                    //         .append_child(&nodes.last().unwrap().as_ref())
                    //         .unwrap();
                    // }
                    Change::Remove(i) => node.remove_child(i),
                    // {
                    //     let node = nodes.remove(i);
                    //     element.remove_child(&node.as_ref()).unwrap();
                    // }
                    Change::Pop => node.pop_child(),
                }
            }
        });

        enclose
    }

    /// Push an element on to the StateVec.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.push(StateAtomic::new(0));
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push(&self, value: T) {
        self.value.write().push(value.clone());
        self.update(Change::Push(value));
    }

    /// Insert an element into the StateVec.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.insert(0, StateAtomic::new(0));
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn insert(&self, index: usize, value: T) {
        self.value.write().insert(index, value.clone());
        self.update(Change::Insert(index, value));
    }

    /// Remove an element from the StateVec by index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    /// vec.remove(3);
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn remove(&self, index: usize) {
        self.value.write().remove(index);
        self.update(Change::Remove(index));
    }

    /// Remove an element from the StateVec using a clone of the element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// let removable = StateAtomic::new(13);
    /// vec.push(removable.clone());
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    ///
    /// vec.remove_elem(removable);
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn remove_elem(&self, elem: T) {
        let index = self.value.read().iter().position(|x| x == &elem).unwrap();
        self.remove(index);
    }

    /// Pop an element from the end of the StateVec.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    /// vec.pop();
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn pop(&self) {
        self.value.write().pop();
        self.update(Change::Pop);
    }

    /// Get an element from the StateVec using index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    ///
    /// div!(
    ///     vec.get(3).unwrap(),
    ///     vec.view(ul!(), |x| li!(x))
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn get(&self, index: usize) -> Option<T> {
        self.value.read().get(index).cloned()
    }

    /// Get the len of the StateVec.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    ///
    /// div!(
    ///     vec.len(),
    ///     vec.view(ul!(), |x| li!(x))
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.value.read().len()
    }

    /// Check whether StateVec is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::with_capacity(10);
    /// (0..10).for_each(|x| vec.push(StateAtomic::new(x)));
    ///
    /// div!(
    ///     vec.is_empty(),
    ///     vec.view(ul!(), |x| li!(x))
    /// )
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.value.read().is_empty()
    }

    fn update(&self, change: Change<T>) {
        self.tx
            .read()
            .iter()
            .map(|x| (x.clone(), change.clone()))
            .for_each(|(tx, change)| {
                wasm_bindgen_futures::spawn_local(async move {
                    tx.send(change).await.unwrap_or(());
                });
            });
    }
}

impl<T> StateVec<StateAtomic<T>>
where
    T: Copy + Send + Display,
{
    /// Push an element on the StateVec.
    /// Same as `push(StateAtomic::new(value))`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.push_atomic(0);
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push_atomic(&self, value: T) {
        let value = StateAtomic::new(value);
        self.push(value);
    }

    /// Push an element on the StateVec.
    /// Same as `push(StateAtomic::insert(index, value))`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.insert_atomic(0, 0);
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn insert_atomic(&self, index: usize, value: T) {
        let value = StateAtomic::new(value);
        self.insert(index, value);
    }
}

impl<T> StateVec<StateMutex<T>>
where
    T: Clone + Send + Display,
{
    /// Push an element on the StateVec.
    /// Same as `push(StateMutex::new(value))`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.push_mutex(0);
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn push_mutex(&self, value: T) {
        let value = StateMutex::new(value);
        self.push(value);
    }

    /// Push an element on the StateVec.
    /// Same as `push(StateMutex::insert(index, value))`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// # fn ui() -> Node {
    /// let vec = StateVec::new();
    /// vec.insert_mutex(0, 0);
    ///
    /// vec.view(ul!(), |x| li!(x))
    /// # .into()
    /// # }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// # fn run() {
    /// #     App::render_single(ui());
    /// # }
    /// ```
    pub fn insert_mutex(&self, index: usize, value: T) {
        let value = StateMutex::new(value);
        self.insert(index, value);
    }
}

impl<T> Clone for StateVec<T>
where
    T: Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            tx: Arc::clone(&self.tx),
        }
    }
}

pub struct IntoIteratorAdapter<T> {
    iter: alloc::vec::IntoIter<T>,
}

impl<T> IntoIterator for StateVec<T>
where
    T: StateTrait + Send,
{
    type Item = T;
    type IntoIter = IntoIteratorAdapter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorAdapter {
            iter: self.value.read().clone().into_iter(),
        }
    }
}

impl<T> Iterator for IntoIteratorAdapter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T> FromIterator<T> for StateVec<T>
where
    T: StateTrait + Send,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        StateVec {
            value: Arc::new(RwLock::new(iter.into_iter().collect())),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<T> FromIterator<T> for StateVec<StateAtomic<T>>
where
    T: Copy + Send + Display,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        StateVec {
            value: Arc::new(RwLock::new(
                iter.into_iter().map(StateAtomic::new).collect(),
            )),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<T> FromIterator<T> for StateVec<StateMutex<T>>
where
    T: Clone + Send + Display,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        StateVec {
            value: Arc::new(RwLock::new(iter.into_iter().map(StateMutex::new).collect())),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

// impl Index for StateVec<T> where T: Send {
//     type Output = &T;
//
//     fn index(&self, index: Idx)
// }
