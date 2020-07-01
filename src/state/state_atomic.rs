use alloc::sync::Arc;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crossbeam::atomic::AtomicCell;
use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};

use crate::channel::Channel;
use crate::component::Component;

use super::StateTrait;

pub struct StateAtomic<T> {
    value: Arc<AtomicCell<T>>,
    tx: StateSender<Channel>,
    rx: StateReceiver<Channel>,
}

impl<T> StateTrait for StateAtomic<T>
where
    T: fmt::Display + Copy,
{
    type Value = T;
    type Pointer = AtomicCell<T>;
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

    fn pointer(&self) -> Arc<Self::Pointer> {
        Arc::clone(&self.value)
    }

    fn update(&self) {
        while self.tx.send(self.value().into()).is_err() {}
    }
}

impl<T> StateAtomic<T> {
    pub fn new(value: T) -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(AtomicCell::new(value)),
            tx,
            rx,
        }
    }
}

impl<T> StateAtomic<T>
where
    T: fmt::Display + Copy,
{
    pub fn from<U, F>(state: &U, mut func: F) -> Self
    where
        U: StateTrait + 'static,
        F: FnMut(U::Value) -> T + 'static,
        T: From<U::Value> + 'static,
    {
        let value = func(state.value());
        let new = Self::new(value);

        super::from(new, state, func)
    }
}

impl<T> StateAtomic<T> where T: fmt::Display + Copy {}

impl<T> Component for StateAtomic<T>
where
    T: fmt::Display + Copy,
{
    type Type = web_sys::Text;

    fn view(self) -> Self::Type {
        let view = self.value().view();
        wasm_bindgen_futures::spawn_local(super::change(view.clone(), self.rx()));

        view
    }
}

impl<T> PartialEq for StateAtomic<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

impl<T> Eq for StateAtomic<T> {}

impl<T> Clone for StateAtomic<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<T, U> Add<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + Add<U> + AddAssign<U>,
{
    type Output = Self;

    fn add(mut self, other: U) -> Self::Output {
        self += other;
        self
    }
}

impl<T, U> AddAssign<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + AddAssign<U>,
{
    fn add_assign(&mut self, other: U) {
        let mut value = self.value();
        value += other;
        self.put(value);
    }
}

impl<T, U> Div<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + Div<U> + DivAssign<U>,
{
    type Output = Self;

    fn div(mut self, other: U) -> Self::Output {
        self /= other;
        self
    }
}

impl<T, U> DivAssign<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + DivAssign<U>,
{
    fn div_assign(&mut self, other: U) {
        let mut value = self.value();
        value /= other;
        self.put(value);
    }
}

impl<T, U> Mul<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + Mul<U> + MulAssign<U>,
{
    type Output = Self;

    fn mul(mut self, other: U) -> Self::Output {
        self *= other;
        self
    }
}

impl<T, U> MulAssign<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + MulAssign<U>,
{
    fn mul_assign(&mut self, other: U) {
        let mut value = self.value();
        value *= other;
        self.put(value);
    }
}

impl<T, U> Rem<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + Rem<U> + RemAssign<U>,
{
    type Output = Self;

    fn rem(mut self, other: U) -> Self::Output {
        self %= other;
        self
    }
}

impl<T, U> RemAssign<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + RemAssign<U>,
{
    fn rem_assign(&mut self, other: U) {
        let mut value = self.value();
        value %= other;
        self.put(value);
    }
}

impl<T, U> Sub<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + Sub<U> + SubAssign<U>,
{
    type Output = Self;

    fn sub(mut self, other: U) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, U> SubAssign<U> for StateAtomic<T>
where
    T: fmt::Display + Copy + SubAssign<U>,
{
    fn sub_assign(&mut self, other: U) {
        let mut value = self.value();
        value -= other;
        self.put(value);
    }
}
