use crate::{channel, Receiver, Sender};
use super::State;

use parking_lot::Mutex;
use futures_intrusive::channel::StateId;
use core::fmt::Display;
use alloc::sync::Arc;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Clone)]
pub struct StateMutex<T>
where
    T: Display + Clone,
{
    value: Arc<Mutex<T>>,
    tx: Arc<Sender>,
    rx: Arc<Receiver>,
}

impl<T> State<T> for StateMutex<T>
where
    T: Display + Clone,
{
    type Value = Mutex<T>;

    fn value(&self) -> T {
        self.value.lock().clone()
    }

    fn tx(&self) -> Arc<Sender> {
        Arc::clone(&self.tx)
    }

    fn rx(&self) -> Arc<Receiver> {
        Arc::clone(&self.rx)
    }

    fn put(&self, value: T) {
        *self.value.lock() = value;
    }

    fn pointer(&self) -> Arc<Self::Value> {
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
        let (tx, rx) = channel();
        Self {
            value: Arc::new(Mutex::new(value)),
            tx: Arc::new(tx),
            rx: Arc::new(rx),
        }
    }

    pub fn from<U, V, F>(state: &U, mut func: F) -> Self
    where
        U: State<V> + 'static,
        V: Display + Clone + 'static,
        F: FnMut(V) -> T + 'static,
        T: From<V> + 'static,
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
