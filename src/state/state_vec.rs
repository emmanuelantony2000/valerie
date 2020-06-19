use super::{StateTrait, StateAtomic, StateMutex};
use crate::Function;

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Display;
use core::iter::FromIterator;
use core::ops::Deref;
use futures_intrusive::channel::shared::{unbuffered_channel, Sender};
use parking_lot::RwLock;

#[derive(Clone)]
enum Change<T>
where
    T: StateTrait + Send,
{
    Insert(usize, T),
    Push(T),
    Remove(usize),
}

#[derive(Clone, Default)]
pub struct StateVec<T>
where
    T: StateTrait + Send + 'static,
{
    value: Arc<RwLock<Vec<T>>>,
    tx: Arc<RwLock<Vec<Sender<Change<T>>>>>,
}

impl<T> StateVec<T>
where
    T: StateTrait + Send,
{
    pub fn new() -> Self {
        Self {
            value: Arc::new(RwLock::new(Vec::new())),
            tx: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn view<F, U, V>(&self, encloser: U, object: F) -> U
    where
        F: FnOnce(T) -> V,
        F: Clone + 'static,
        U: Deref<Target = Function>,
        V: Deref<Target = Function> + 'static,
    {
        let mut nodes = Vec::with_capacity(self.len());
        for i in self.value.read().iter() {
            nodes.push(object.clone()(i.clone()));
            encloser.append_child(&nodes.last().unwrap()).unwrap();
        }

        if let Some(element) = encloser.element() {
            let (tx, rx) = unbuffered_channel();
            self.tx.write().push(tx);
            wasm_bindgen_futures::spawn_local(async move {
                while let Some(change) = rx.receive().await {
                    match change {
                        Change::Insert(i, x) => {
                            nodes.insert(i, object.clone()(x));
                            element
                                .insert_before(
                                    &nodes[i],
                                    Some(&element.children().item(i as u32).unwrap()),
                                )
                                .unwrap();
                        }

                        Change::Push(x) => {
                            nodes.push(object.clone()(x));
                            element.append_child(&nodes.last().unwrap()).unwrap();
                        }

                        Change::Remove(i) => {
                            let node = nodes.remove(i);
                            element.remove_child(&node).unwrap();
                        }
                    }
                }
            });
        }

        encloser
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
        self.tx.read().iter().for_each(|tx| {
            let tx = tx.clone();
            let change = change.clone();
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
