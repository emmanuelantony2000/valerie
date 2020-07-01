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

pub mod state_atomic;
pub mod state_builder;
pub mod state_mutex;
pub mod state_vec;

pub trait StateTrait: Clone + Component + Eq {
    type Value: Clone + Display;
    type Pointer;
    type Channel: Clone;

    fn value(&self) -> Self::Value;
    fn tx(&self) -> StateSender<Self::Channel>;
    fn rx(&self) -> StateReceiver<Self::Channel>;
    fn put(&self, value: Self::Value);
    fn pointer(&self) -> Arc<Self::Pointer>;
    fn update(&self);
}

pub fn from<T, U, F>(new: T, state: &U, mut func: F) -> T
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

pub async fn change(node: impl AsRef<web_sys::Node>, rx: StateReceiver<Channel>) {
    let mut old = StateId::new();
    while let Some((new, value)) = rx.receive(old).await {
        node.as_ref().set_node_value(Some(&value));
        old = new;
    }
}
