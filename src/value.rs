use alloc::boxed::Box;
use alloc::string::ToString;
use core::fmt;

use crate::state::{self, StateAtomic, StateGeneric, StateMutex, StateTrait};

/// `Value` trait
///
/// Implement this so that the type can be used to bind themselves to attributes.
pub trait Value {
    /// The `bind_func` has to call the `func` function with the parameter
    /// as its own value whenever it is updated.
    /// Ignore the repeated calls if your type is not a state variable.
    fn bind_func(&self, func: Box<dyn FnMut(&str)>);
}

impl<T> Value for T
where
    T: fmt::Display,
{
    fn bind_func(&self, mut func: Box<dyn FnMut(&str)>) {
        func(self.to_string().as_str());
    }
}

impl<T> Value for StateAtomic<T>
where
    T: Copy + fmt::Display,
{
    fn bind_func(&self, func: Box<dyn FnMut(&str)>) {
        wasm_bindgen_futures::spawn_local(state::state_change(func, self.rx()));
    }
}

impl<T> Value for StateMutex<T>
where
    T: Clone + fmt::Display,
{
    fn bind_func(&self, func: Box<dyn FnMut(&str)>) {
        wasm_bindgen_futures::spawn_local(state::state_change(func, self.rx()));
    }
}

impl<T, D> Value for StateGeneric<T, D>
where
    D: Clone + fmt::Display,
{
    fn bind_func(&self, func: Box<dyn FnMut(&str)>) {
        wasm_bindgen_futures::spawn_local(state::state_change(func, self.rx()));
    }
}
