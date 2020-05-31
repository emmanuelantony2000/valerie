use crate::{Receiver, Sender};

use alloc::sync::Arc;
use core::fmt::Display;

pub use state_atomic::StateAtomic;
pub use state_mutex::StateMutex;

pub mod state_atomic;
pub mod state_mutex;

pub trait State<T>: Clone
where
    T: Display + Clone,
{
    type Value;

    fn value(&self) -> T;
    fn tx(&self) -> Arc<Sender>;
    fn rx(&self) -> Arc<Receiver>;
    fn put(&self, value: T);
    fn pointer(&self) -> Arc<Self::Value>;
    fn update(&self);
}
