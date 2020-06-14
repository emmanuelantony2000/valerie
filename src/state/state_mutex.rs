use crate::{Channel, Component, Function};
use super::State;

use parking_lot::Mutex;
use futures_intrusive::channel::StateId;
use core::fmt::Display;
use futures_intrusive::channel::shared::{state_broadcast_channel, StateSender, StateReceiver};
use alloc::sync::Arc;
use web_sys::{Node};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Clone)]
pub struct StateMutex<T>
where
    T: Display + Clone,
{
    value: Arc<Mutex<T>>,
    tx: StateSender<Channel>,
    rx: StateReceiver<Channel>,
}

impl<T> State for StateMutex<T>
where
    T: Display + Clone,
{
    type Value = T;
    type Pointer = Arc<Mutex<T>>;
    type Channel = Channel;

    fn value(&self) -> Self::Value {
        self.value.lock().clone()
    }

    fn tx(&self) -> StateSender<Self::Channel> {
        self.tx.clone()
    }

    fn rx(&self) -> StateReceiver<Self::Channel> {
        self.rx.clone()
    }

    fn put(&self, value: Self::Value) {
        *self.value.lock() = value;
    }

    fn pointer(&self) -> Self::Pointer {
        Arc::clone(&self.value)
    }

    fn update(&self) {
        while self.tx.send(self.value.lock().into()).is_err() {}
    }
}

impl<T> StateMutex<T>
where
    T: Display + Clone,
{
    pub fn new(value: T) -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(Mutex::new(value)),
            tx,
            rx,
        }
    }

    pub fn from<U, F>(state: &U, mut func: F) -> Self
    where
        U: State + 'static,
        F: FnMut(U::Value) -> T + 'static,
        T: From<U::Value> + 'static,
    {
        let value = func(state.value());
        let new = Self::new(value);

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
}

impl<T> Component for StateMutex<T> where T: Display + Clone {
    fn view(self) -> Function {
        let function = self.value.lock().view();
        // function.rx = Some(self.rx());
        wasm_bindgen_futures::spawn_local(change(function.node().clone(), self.rx()));
        function
    }
}

pub async fn change(node: Node, rx: StateReceiver<Channel>) {
    let mut old = StateId::new();
    while let Some((new, value)) = rx.receive(old).await {
        node.set_node_value(Some(&value));
        old = new;
    }
}

impl<T, U> Add<U> for StateMutex<T>
where
    T: Display + Clone + Add<U> + AddAssign<U>,
{
    type Output = Self;

    fn add(mut self, other: U) -> Self::Output {
        self += other;
        self
    }
}

impl<T, U> AddAssign<U> for StateMutex<T>
where
    T: Display + Clone + AddAssign<U>,
{
    fn add_assign(&mut self, other: U) {
        *self.value.lock() += other;
    }
}

impl<T, U> Div<U> for StateMutex<T>
where
    T: Display + Clone + Div<U> + DivAssign<U>,
{
    type Output = Self;

    fn div(mut self, other: U) -> Self::Output {
        self /= other;
        self
    }
}

impl<T, U> DivAssign<U> for StateMutex<T>
where
    T: Display + Clone + DivAssign<U>,
{
    fn div_assign(&mut self, other: U) {
        *self.value.lock() /= other;
    }
}

impl<T, U> Mul<U> for StateMutex<T>
where
    T: Display + Clone + Mul<U> + MulAssign<U>,
{
    type Output = Self;

    fn mul(mut self, other: U) -> Self::Output {
        self *= other;
        self
    }
}

impl<T, U> MulAssign<U> for StateMutex<T>
where
    T: Display + Clone + MulAssign<U>,
{
    fn mul_assign(&mut self, other: U) {
        *self.value.lock() *= other;
    }
}

impl<T, U> Rem<U> for StateMutex<T>
where
    T: Display + Clone + Rem<U> + RemAssign<U>,
{
    type Output = Self;

    fn rem(mut self, other: U) -> Self::Output {
        self %= other;
        self
    }
}

impl<T, U> RemAssign<U> for StateMutex<T>
where
    T: Display + Clone + RemAssign<U>,
{
    fn rem_assign(&mut self, other: U) {
        *self.value.lock() %= other;
    }
}

impl<T, U> Sub<U> for StateMutex<T>
where
    T: Display + Clone + Sub<U> + SubAssign<U>,
{
    type Output = Self;

    fn sub(mut self, other: U) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, U> SubAssign<U> for StateMutex<T>
where
    T: Display + Clone + SubAssign<U>,
{
    fn sub_assign(&mut self, other: U) {
        *self.value.lock() -= other;
    }
}
