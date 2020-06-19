use super::StateTrait;
use crate::{Channel, Component, Function};

use alloc::sync::Arc;
use core::fmt::Display;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
use futures_intrusive::channel::StateId;
use web_sys::Node;

pub struct StateBuilder<T, R, W, D> {
    value: Arc<T>,
    reader: R,
    writer: W,
    tx: StateSender<Channel>,
    rx: StateReceiver<Channel>,
    _data: PhantomData<D>
}

impl<T, R, W, D> Clone for StateBuilder<T, R, W, D>
where 
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone,
    W: Fn(&T, D),
    W: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            reader: self.reader.clone(),
            writer: self.writer.clone(),
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            _data: PhantomData,
        }
    }
}

impl<T, R, W, D> StateTrait for StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone,
    W: Fn(&T, D),
    W: Clone,
{
    type Value = D;
    type Pointer = Arc<T>;
    type Channel = Channel;

    fn value(&self) -> Self::Value {
        self.reader.clone()(&self.value)
    }

    fn tx(&self) -> StateSender<Self::Channel> {
        self.tx.clone()
    }

    fn rx(&self) -> StateReceiver<Self::Channel> {
        self.rx.clone()
    }

    fn put(&self, value: Self::Value) {
        self.writer.clone()(&self.value, value);
        self.update();
    }

    fn pointer(&self) -> Self::Pointer {
        Arc::clone(&self.value)
    }
}

impl<T, R, W, D> StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone,
    W: Fn(&T, D),
    W: Clone,
{
    pub fn new(value: T, reader: R, writer: W) -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(value),
            reader,
            writer,
            tx,
            rx,
            _data: PhantomData,
        }
    }

    // pub fn from<U, F>(state: &U, mut func: F) -> Self
    // where
    //     U: StateTrait + 'static,
    //     F: FnMut(U::Value) -> T + 'static,
    //     T: From<U::Value> + 'static,
    // {
    //     let value = func(state.value());
    //     let new = Self::new(value);

    //     let new_move = new.clone();
    //     let state_value = state.clone();
    //     let rx = state.rx();
    //     wasm_bindgen_futures::spawn_local(async move {
    //         let mut old = StateId::new();
    //         while let Some((new, _)) = rx.receive(old).await {
    //             new_move.put(func(state_value.value()));
    //             new_move.update();

    //             old = new;
    //         }
    //     });

    //     new
    // }

    fn update(&self) {
        while self.tx.send(self.value().into()).is_err() {}
    }
}

impl<T, R, W, D> Component for StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone,
    W: Fn(&T, D),
    W: Clone,
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

impl<T, R, W, D, U> Add<U> for StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone + Add<U> + AddAssign<U>,
    W: Fn(&T, D),
    W: Clone,
{
    type Output = Self;

    fn add(mut self, other: U) -> Self::Output {
        self += other;
        self
    }
}

impl<T, R, W, D, U> AddAssign<U> for StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone + AddAssign<U>,
    W: Fn(&T, D),
    W: Clone,
{
    fn add_assign(&mut self, other: U) {
        let mut value = self.value();
        value += other;
        self.put(value);
    }
}

impl<T, R, W, D, U> Div<U> for StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone + Div<U> + DivAssign<U>,
    W: Fn(&T, D),
    W: Clone,
{
    type Output = Self;

    fn div(mut self, other: U) -> Self::Output {
        self /= other;
        self
    }
}

impl<T, R, W, D, U> DivAssign<U> for StateBuilder<T, R, W, D>
where
    R: Fn(&T) -> D,
    R: Clone,
    D: Display + Clone + DivAssign<U>,
    W: Fn(&T, D),
    W: Clone,
{
    fn div_assign(&mut self, other: U) {
        let mut value = self.value();
        value /= other;
        self.put(value);
    }
}

impl<T, R, W, D, U> Mul<U> for StateBuilder<T, R, W, D>
where
R: Fn(&T) -> D,
R: Clone,
D: Display + Clone + Mul<U> + MulAssign<U>,
W: Fn(&T, D),
W: Clone,
{
    type Output = Self;

    fn mul(mut self, other: U) -> Self::Output {
        self *= other;
        self
    }
}

impl<T, R, W, D, U> MulAssign<U> for StateBuilder<T, R, W, D>
where
R: Fn(&T) -> D,
R: Clone,
D: Display + Clone + MulAssign<U>,
W: Fn(&T, D),
W: Clone,
{
    fn mul_assign(&mut self, other: U) {
        let mut value = self.value();
        value *= other;
        self.put(value);
    }
}

impl<T, R, W, D, U> Rem<U> for StateBuilder<T, R, W, D>
where
R: Fn(&T) -> D,
R: Clone,
D: Display + Clone + Rem<U> + RemAssign<U>,
W: Fn(&T, D),
W: Clone,
{
    type Output = Self;

    fn rem(mut self, other: U) -> Self::Output {
        self %= other;
        self
    }
}

impl<T, R, W, D, U> RemAssign<U> for StateBuilder<T, R, W, D>
where
R: Fn(&T) -> D,
R: Clone,
D: Display + Clone + RemAssign<U>,
W: Fn(&T, D),
W: Clone,
{
    fn rem_assign(&mut self, other: U) {
        let mut value = self.value();
        value %= other;
        self.put(value);
    }
}

impl<T, R, W, D, U> Sub<U> for StateBuilder<T, R, W, D>
where
R: Fn(&T) -> D,
R: Clone,
D: Display + Clone + Sub<U> + SubAssign<U>,
W: Fn(&T, D),
W: Clone,
{
    type Output = Self;

    fn sub(mut self, other: U) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, R, W, D, U> SubAssign<U> for StateBuilder<T, R, W, D>
where
R: Fn(&T) -> D,
R: Clone,
D: Display + Clone + SubAssign<U>,
W: Fn(&T, D),
W: Clone,
{
    fn sub_assign(&mut self, other: U) {
        let mut value = self.value();
        value -= other;
        self.put(value);
    }
}
