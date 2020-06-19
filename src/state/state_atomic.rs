use super::StateTrait;
use crate::{Channel, Component, Function};

use alloc::sync::Arc;
use core::fmt::Display;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use crossbeam::atomic::AtomicCell;
use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
use futures_intrusive::channel::StateId;
use web_sys::Node;

#[derive(Clone)]
pub struct StateAtomic<T>
where
    T: Display + Copy,
{
    value: Arc<AtomicCell<T>>,
    tx: StateSender<Channel>,
    rx: StateReceiver<Channel>,
}

impl<T> StateTrait for StateAtomic<T>
where
    T: Display + Copy,
{
    type Value = T;
    type Pointer = Arc<AtomicCell<T>>;
    type Channel = Channel;

    fn value(&self) -> Self::Value {
        self.value.load()
    }

    fn tx(&self) -> StateSender<Self::Channel> {
        self.tx.clone()
    }

    fn rx(&self) -> StateReceiver<Self::Channel> {
        self.rx.clone()
    }

    fn put(&self, value: Self::Value) {
        self.value.store(value);
        self.update();
    }

    fn pointer(&self) -> Self::Pointer {
        Arc::clone(&self.value)
    }
}

impl<T> StateAtomic<T>
where
    T: Display + Copy,
{
    pub fn new(value: T) -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(AtomicCell::new(value)),
            tx,
            rx,
        }
    }

    pub fn from<U, F>(state: &U, mut func: F) -> Self
    where
        U: StateTrait + 'static,
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

    fn update(&self) {
        while self.tx.send(self.value().into()).is_err() {}
    }
}

impl<T> Component for StateAtomic<T>
where
    T: Display + Copy,
{
    fn view(self) -> Function {
        let function = self.value().view();
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

impl<T, U> Add<U> for StateAtomic<T>
where
    T: Display + Copy + Add<U> + AddAssign<U>,
{
    type Output = Self;

    fn add(mut self, other: U) -> Self::Output {
        self += other;
        self
    }
}

impl<T, U> AddAssign<U> for StateAtomic<T>
where
    T: Display + Copy + AddAssign<U>,
{
    fn add_assign(&mut self, other: U) {
        let mut value = self.value();
        value += other;
        self.put(value);
    }
}

impl<T, U> Div<U> for StateAtomic<T>
where
    T: Display + Copy + Div<U> + DivAssign<U>,
{
    type Output = Self;

    fn div(mut self, other: U) -> Self::Output {
        self /= other;
        self
    }
}

impl<T, U> DivAssign<U> for StateAtomic<T>
where
    T: Display + Copy + DivAssign<U>,
{
    fn div_assign(&mut self, other: U) {
        let mut value = self.value();
        value /= other;
        self.put(value);
    }
}

impl<T, U> Mul<U> for StateAtomic<T>
where
    T: Display + Copy + Mul<U> + MulAssign<U>,
{
    type Output = Self;

    fn mul(mut self, other: U) -> Self::Output {
        self *= other;
        self
    }
}

impl<T, U> MulAssign<U> for StateAtomic<T>
where
    T: Display + Copy + MulAssign<U>,
{
    fn mul_assign(&mut self, other: U) {
        let mut value = self.value();
        value *= other;
        self.put(value);
    }
}

impl<T, U> Rem<U> for StateAtomic<T>
where
    T: Display + Copy + Rem<U> + RemAssign<U>,
{
    type Output = Self;

    fn rem(mut self, other: U) -> Self::Output {
        self %= other;
        self
    }
}

impl<T, U> RemAssign<U> for StateAtomic<T>
where
    T: Display + Copy + RemAssign<U>,
{
    fn rem_assign(&mut self, other: U) {
        let mut value = self.value();
        value %= other;
        self.put(value);
    }
}

impl<T, U> Sub<U> for StateAtomic<T>
where
    T: Display + Copy + Sub<U> + SubAssign<U>,
{
    type Output = Self;

    fn sub(mut self, other: U) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, U> SubAssign<U> for StateAtomic<T>
where
    T: Display + Copy + SubAssign<U>,
{
    fn sub_assign(&mut self, other: U) {
        let mut value = self.value();
        value -= other;
        self.put(value);
    }
}
