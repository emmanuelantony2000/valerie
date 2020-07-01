use alloc::sync::Arc;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
use parking_lot::Mutex;

use crate::channel::Channel;
use crate::component::Component;

use super::StateTrait;

pub struct StateMutex<T> {
    value: Arc<Mutex<T>>,
    tx: StateSender<Channel>,
    rx: StateReceiver<Channel>,
}

impl<T> StateTrait for StateMutex<T>
where
    T: fmt::Display + Clone,
{
    type Value = T;
    type Pointer = Mutex<T>;
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
        self.update();
    }

    fn pointer(&self) -> Arc<Self::Pointer> {
        Arc::clone(&self.value)
    }

    fn update(&self) {
        while self.tx.send(self.value.lock().into()).is_err() {}
    }
}

impl<T> StateMutex<T> {
    pub fn new(value: T) -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(Mutex::new(value)),
            tx,
            rx,
        }
    }
}

impl<T> StateMutex<T>
where
    T: fmt::Display + Clone,
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

impl<T> Component for StateMutex<T>
where
    T: fmt::Display + Clone,
{
    type Type = web_sys::Text;

    fn view(self) -> Self::Type {
        let elem = self.value.lock().view();
        wasm_bindgen_futures::spawn_local(super::change(elem.clone(), self.rx()));

        elem
    }
}

impl<T> PartialEq for StateMutex<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

impl<T> Eq for StateMutex<T> {}

impl<T> Clone for StateMutex<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<T, U> Add<U> for StateMutex<T>
where
    T: fmt::Display + Clone + Add<U> + AddAssign<U>,
{
    type Output = Self;

    fn add(mut self, other: U) -> Self::Output {
        self += other;
        self
    }
}

impl<T, U> AddAssign<U> for StateMutex<T>
where
    T: fmt::Display + Clone + AddAssign<U>,
{
    fn add_assign(&mut self, other: U) {
        *self.value.lock() += other;
    }
}

impl<T, U> Div<U> for StateMutex<T>
where
    T: fmt::Display + Clone + Div<U> + DivAssign<U>,
{
    type Output = Self;

    fn div(mut self, other: U) -> Self::Output {
        self /= other;
        self
    }
}

impl<T, U> DivAssign<U> for StateMutex<T>
where
    T: fmt::Display + Clone + DivAssign<U>,
{
    fn div_assign(&mut self, other: U) {
        *self.value.lock() /= other;
    }
}

impl<T, U> Mul<U> for StateMutex<T>
where
    T: fmt::Display + Clone + Mul<U> + MulAssign<U>,
{
    type Output = Self;

    fn mul(mut self, other: U) -> Self::Output {
        self *= other;
        self
    }
}

impl<T, U> MulAssign<U> for StateMutex<T>
where
    T: fmt::Display + Clone + MulAssign<U>,
{
    fn mul_assign(&mut self, other: U) {
        *self.value.lock() *= other;
    }
}

impl<T, U> Rem<U> for StateMutex<T>
where
    T: fmt::Display + Clone + Rem<U> + RemAssign<U>,
{
    type Output = Self;

    fn rem(mut self, other: U) -> Self::Output {
        self %= other;
        self
    }
}

impl<T, U> RemAssign<U> for StateMutex<T>
where
    T: fmt::Display + Clone + RemAssign<U>,
{
    fn rem_assign(&mut self, other: U) {
        *self.value.lock() %= other;
    }
}

impl<T, U> Sub<U> for StateMutex<T>
where
    T: fmt::Display + Clone + Sub<U> + SubAssign<U>,
{
    type Output = Self;

    fn sub(mut self, other: U) -> Self::Output {
        self -= other;
        self
    }
}

impl<T, U> SubAssign<U> for StateMutex<T>
where
    T: fmt::Display + Clone + SubAssign<U>,
{
    fn sub_assign(&mut self, other: U) {
        *self.value.lock() -= other;
    }
}
