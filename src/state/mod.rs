use crate::{Component, Function};

use alloc::sync::Arc;
use core::fmt::Display;
use futures_intrusive::channel::shared::{StateReceiver, StateSender};

pub use state_atomic::StateAtomic;
pub use state_mutex::StateMutex;
pub use state_vec::StateVec;

pub mod state_atomic;
pub mod state_mutex;
pub mod state_vec;

pub trait State: Clone + Component
{
    type Value: Clone + Display;
    type Pointer;
    type Channel: Clone;

    fn value(&self) -> Self::Value;
    fn tx(&self) -> StateSender<Self::Channel>;
    fn rx(&self) -> StateReceiver<Self::Channel>;
    fn put(&self, value: Self::Value);
    fn pointer(&self) -> Self::Pointer;
    fn update(&self);
}

