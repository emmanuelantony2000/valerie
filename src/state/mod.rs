//!
//! State variables handles the core functionality in valerie.
//!
//! When a change happens, the change is propagated to all the elements using the state variables
//! using channels. The value passed is `Channel`. Whenever the update function is called,
//! the State variable updates its value across all the places in the DOM.
//!
//! This is achieved using message passing concurrency. Each place where the state is used,
//! It also spawns an async function with the receiver that waits for a message from the sender.
//! When the sender sends the message receiver updates the value associated.

use alloc::sync::Arc;
use core::fmt::Display;

use futures_intrusive::channel::shared::{StateReceiver, StateSender};
use futures_intrusive::channel::StateId;

pub use state_atomic::StateAtomic;
// pub use state_builder::StateBuilder;
pub use state_mutex::StateMutex;
pub use state_vec::StateVec;

use crate::channel::Channel;
use crate::component::Component;

mod state_atomic;
mod state_builder;
mod state_mutex;
mod state_vec;

/// Trait that State types have to implement
pub trait StateTrait: Clone + Component + Eq {
    /// Type that is stored in the State variable.
    type Value: Clone + Display;

    /// Type that is wrapped in `Arc` which stores the value.
    type Store;

    /// Channel type is used to pass the message between the Sender and the Receiver.
    type Channel: Clone;

    /// Returns a copy of the original value stored.
    fn value(&self) -> Self::Value;

    /// Returns the clone of the Sender.
    fn tx(&self) -> StateSender<Self::Channel>;

    /// Returns the clone of the Receiver.
    fn rx(&self) -> StateReceiver<Self::Channel>;

    /// Stores the value on to the State variable.
    fn put(&self, value: Self::Value);

    /// Gives a pointer to the value.
    fn pointer(&self) -> Arc<Self::Store>;

    /// Update the value across the DOM. `put()` should call this.
    fn update(&self);
}

pub(crate) fn from<T, U, F>(new: T, state: &U, mut func: F) -> T
where
    T: StateTrait + 'static,
    U: StateTrait + 'static,
    F: FnMut(U::Value) -> T::Value + 'static,
    T::Value: From<U::Value> + 'static,
{
    let new_move = new.clone();
    let state_value = state.clone();
    let rx = state.rx();
    wasm_bindgen_futures::spawn_local(async move {
        let mut old = StateId::new();
        while let Some((new, _)) = rx.receive(old).await {
            new_move.put(func(state_value.value()));
            new_move.update();

            old = new;
        }
    });

    new
}

pub(crate) async fn change(node: impl AsRef<web_sys::Node>, rx: StateReceiver<Channel>) {
    let mut old = StateId::new();
    while let Some((new, value)) = rx.receive(old).await {
        node.as_ref().set_node_value(Some(&value));
        old = new;
    }
}
