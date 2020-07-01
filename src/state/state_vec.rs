use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Display;
use core::iter::FromIterator;
// use core::ops::{Index, IndexMut};

use futures_intrusive::channel::shared::{unbuffered_channel, Sender};
use parking_lot::RwLock;

use super::{StateAtomic, StateMutex, StateTrait};

#[derive(Clone)]
enum Change<T>
where
    T: Send,
{
    Insert(usize, T),
    Push(T),
    Remove(usize),
}

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
    pub fn new() -> Self {
        Self {
            value: Arc::new(RwLock::new(Vec::new())),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<T> StateVec<T>
where
    T: StateTrait + Send,
{
    pub fn view<F, U, V>(&self, enclose: U, object: F) -> U
    where
        F: FnOnce(T) -> V,
        F: Clone + 'static,
        U: AsRef<web_sys::Node>,
        V: AsRef<web_sys::Node> + 'static,
    {
        let mut nodes = Vec::with_capacity(self.len());
        for i in self.value.read().iter() {
            nodes.push(object.clone()(i.clone()));
            enclose
                .as_ref()
                .append_child(nodes.last().unwrap().as_ref())
                .unwrap();
        }

        let element = enclose.as_ref().clone();
        let (tx, rx) = unbuffered_channel();
        self.tx.write().push(tx);
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(change) = rx.receive().await {
                match change {
                    Change::Insert(i, x) => {
                        nodes.insert(i, object.clone()(x));
                        element
                            .insert_before(&nodes[i].as_ref(), Some(&nodes[i + 1].as_ref()))
                            .unwrap();
                    }

                    Change::Push(x) => {
                        nodes.push(object.clone()(x));
                        element
                            .append_child(&nodes.last().unwrap().as_ref())
                            .unwrap();
                    }

                    Change::Remove(i) => {
                        let node = nodes.remove(i);
                        element.remove_child(&node.as_ref()).unwrap();
                    }
                }
            }
        });

        enclose
    }

    pub fn push(&self, value: T) {
        self.value.write().push(value.clone());
        self.update(Change::Push(value));
    }

    pub fn insert(&self, index: usize, value: T) {
        self.value.write().insert(index, value.clone());
        self.update(Change::Insert(index, value));
    }

    pub fn remove(&self, index: usize) {
        self.value.write().remove(index);
        self.update(Change::Remove(index));
    }

    pub fn remove_elem(&self, elem: T) {
        web_sys::console::log_1(&alloc::format!("Elem {}", elem.value()).into());
        web_sys::console::log_1(&alloc::format!("Equals").into());
        self.value
            .read()
            .iter()
            .for_each(|x| web_sys::console::log_1(&alloc::format!("{}", x == &elem).into()));

        let index = self.value.read().iter().position(|x| x == &elem).unwrap();
        self.remove(index);
    }

    pub fn pop(&self) {
        self.value.write().pop();
        self.update(Change::Remove(self.len()));
    }

    pub fn len(&self) -> usize {
        self.value.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.read().is_empty()
    }

    fn update(&self, change: Change<T>) {
        self.value
            .read()
            .iter()
            .for_each(|x| web_sys::console::log_1(&alloc::format!("{}", x.value()).into()));
        web_sys::console::log_1(&alloc::format!("That's it!").into());

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
    pub fn push_atomic(&self, value: T) {
        self.value.write().push(StateAtomic::new(value));
        self.update(Change::Push(StateAtomic::new(value)));
    }

    pub fn insert_atomic(&self, index: usize, value: T) {
        self.value.write().insert(index, StateAtomic::new(value));
        self.update(Change::Insert(index, StateAtomic::new(value)));
    }
}

impl<T> StateVec<StateMutex<T>>
where
    T: Clone + Send + Display,
{
    pub fn push_mutex(&self, value: T) {
        self.value.write().push(StateMutex::new(value.clone()));
        self.update(Change::Push(StateMutex::new(value)));
    }

    pub fn insert_mutex(&self, index: usize, value: T) {
        self.value
            .write()
            .insert(index, StateMutex::new(value.clone()));
        self.update(Change::Insert(index, StateMutex::new(value)));
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
// }f
