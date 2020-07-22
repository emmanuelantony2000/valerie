use alloc::sync::Arc;
use core::fmt;

use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};

use crate::channel::Channel;
use crate::component::Component;

use super::StateTrait;

/// A generic State type
///
/// You can make your own type of State Variable using this.
pub struct StateGeneric<T, D> {
    value: Arc<T>,
    reader: fn(&T) -> D,
    writer: fn(&T, D),
    new: fn(D) -> T,
    tx: StateSender<Channel>,
    rx: StateReceiver<Channel>,
}

impl<T, D> StateTrait for StateGeneric<T, D>
where
    D: fmt::Display + Clone,
{
    type Value = D;
    type Store = T;
    type Channel = Channel;

    fn value(&self) -> Self::Value {
        (self.reader)(&self.value)
    }

    fn tx(&self) -> StateSender<Self::Channel> {
        self.tx.clone()
    }

    fn rx(&self) -> StateReceiver<Self::Channel> {
        self.rx.clone()
    }

    fn put(&self, value: Self::Value) {
        (self.writer)(&self.value, value);
        self.update();
    }

    fn pointer(&self) -> Arc<Self::Store> {
        Arc::clone(&self.value)
    }

    fn update(&self) {
        while self.tx.send(self.value().into()).is_err() {}
    }
}

impl<T, D> StateGeneric<T, D> {
    /// To make a new Generic State type.
    ///
    /// Description
    ///
    ///  * `value` - The data to store
    ///  * `reader` - The reader function which reads the function
    ///  * `writer` - The writer function
    ///  * `new` - To make enclose the `value`, eg: Mutex::new
    pub fn new(value: D, reader: fn(&T) -> D, writer: fn(&T, D), new: fn(D) -> T) -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(new(value)),
            reader,
            writer,
            new,
            tx,
            rx,
        }
    }
}

impl<T, D> StateGeneric<T, D>
where
    T: 'static,
    D: fmt::Display + Clone + 'static,
{
    /// A from function to derive the value from other state variables.
    pub fn from<U, F>(
        state: &U,
        mut func: F,
        reader: fn(&T) -> D,
        writer: fn(&T, D),
        new: fn(D) -> T,
    ) -> Self
    where
        U: StateTrait + 'static,
        F: FnMut(U::Value) -> <Self as StateTrait>::Value + 'static,
    {
        let value = func(state.value());
        let new = Self::new(value, reader, writer, new);

        super::from(new, state, func)
    }
}

impl<T, D> Component for StateGeneric<T, D> where D: fmt::Display + Clone {}

impl<T, D> From<StateGeneric<T, D>> for crate::Node
where
    D: fmt::Display + Clone,
{
    fn from(x: StateGeneric<T, D>) -> Self {
        let elem: Self = x.value().into();
        wasm_bindgen_futures::spawn_local(super::change(elem.clone(), x.rx()));

        elem
    }
}

impl<T, D> PartialEq for StateGeneric<T, D> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

impl<T, D> Eq for StateGeneric<T, D> {}

impl<T, D> Clone for StateGeneric<T, D> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            reader: self.reader,
            writer: self.writer,
            new: self.new,
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}
