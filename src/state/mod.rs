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

// impl<T, U> Add<U> for State<T>
// where
//     T: Display + Clone + Add<U> + AddAssign<U>,
// {
//     type Output = Self;

//     fn add(mut self, other: U) -> Self::Output {
//         self += other;
//         self
//     }
// }

// impl<T, U> AddAssign<U> for State<T>
// where
//     T: Display + Clone + AddAssign<U>,
// {
//     fn add_assign(&mut self, other: U) {
//         *self.value().lock() += other;
//     }
// }

// impl<T, U> Div<U> for State<T>
// where
//     T: Display + Clone + Div<U> + DivAssign<U>,
// {
//     type Output = Self;

//     fn div(mut self, other: U) -> Self::Output {
//         self /= other;
//         self
//     }
// }

// impl<T, U> DivAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: DivAssign<U>,
// {
//     fn div_assign(&mut self, other: U) {
//         *self.value().lock() /= other;
//     }
// }

// impl<T, U> Mul<U> for State<T>
// where
//     T: Display + Clone + Mul<U> + MulAssign<U>,
// {
//     type Output = Self;

//     fn mul(mut self, other: U) -> Self::Output {
//         self *= other;
//         self
//     }
// }

// impl<T, U> MulAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: MulAssign<U>,
// {
//     fn mul_assign(&mut self, other: U) {
//         *self.value().lock() *= other;
//     }
// }

// impl<T, U> Rem<U> for State<T>
// where
//     T: Display + Clone + Rem<U> + RemAssign<U>,
// {
//     type Output = Self;

//     fn rem(mut self, other: U) -> Self::Output {
//         self %= other;
//         self
//     }
// }

// impl<T, U> RemAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: RemAssign<U>,
// {
//     fn rem_assign(&mut self, other: U) {
//         *self.value().lock() %= other;
//     }
// }

// impl<T, U> Sub<U> for State<T>
// where
//     T: Display + Clone + Sub<U> + SubAssign<U>,
// {
//     type Output = Self;

//     fn sub(mut self, other: U) -> Self::Output {
//         self -= other;
//         self
//     }
// }

// impl<T, U> SubAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: SubAssign<U>,
// {
//     fn sub_assign(&mut self, other: U) {
//         *self.value().lock() -= other;
//     }
// }

// #[derive(Clone)]
// pub struct State<T>
// where
//     T: Display + Clone,
// {
//     value: Arc<Mutex<T>>,
//     tx: Arc<Sender>,
//     rx: Arc<Receiver>,
// }

// impl<T> State<T>
// where
//     T: Display + Clone,
// {
//     pub fn new(value: T) -> Self {
//         let (tx, rx) = channel();
//         Self {
//             value: Arc::new(Mutex::new(value)),
//             tx: Arc::new(tx),
//             rx: Arc::new(rx),
//         }
//     }

//     pub fn from<U, F>(state: &State<U>, mut func: F) -> Self
//     where
//         U: Display + Clone + 'static,
//         F: FnMut(U) -> T + 'static,
//         T: From<U> + 'static,
//     {
//         let value = func(state.value.lock().clone());
//         let new = Self::new(value);

//         let new_move = new.clone();
//         let state_value = state.value();
//         let rx = state.rx();
//         wasm_bindgen_futures::spawn_local(async move {
//             let mut old = StateId::new();
//             while let Some((new, _)) = rx.receive(old).await {
//                 *new_move.value.lock() = func(state_value.lock().clone());
//                 new_move.update();

//                 old = new;
//             }
//         });

//         new
//     }

//     pub fn update(&self) {
//         while self.tx.send(self.value().lock().into()).is_err() {}
//     }

//     pub fn tx(&self) -> Arc<Sender> {
//         Arc::clone(&self.tx)
//     }

//     pub fn rx(&self) -> Arc<Receiver> {
//         Arc::clone(&self.rx)
//     }

//     pub fn value(&self) -> Arc<Mutex<T>> {
//         Arc::clone(&self.value)
//     }
// }

// impl<T, U> Add<U> for State<T>
// where
//     T: Display + Clone + Add<U> + AddAssign<U>,
// {
//     type Output = Self;

//     fn add(mut self, other: U) -> Self::Output {
//         self += other;
//         self
//     }
// }

// impl<T, U> AddAssign<U> for State<T>
// where
//     T: Display + Clone + AddAssign<U>,
// {
//     fn add_assign(&mut self, other: U) {
//         *self.value().lock() += other;
//     }
// }

// impl<T, U> Div<U> for State<T>
// where
//     T: Display + Clone + Div<U> + DivAssign<U>,
// {
//     type Output = Self;

//     fn div(mut self, other: U) -> Self::Output {
//         self /= other;
//         self
//     }
// }

// impl<T, U> DivAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: DivAssign<U>,
// {
//     fn div_assign(&mut self, other: U) {
//         *self.value().lock() /= other;
//     }
// }

// impl<T, U> Mul<U> for State<T>
// where
//     T: Display + Clone + Mul<U> + MulAssign<U>,
// {
//     type Output = Self;

//     fn mul(mut self, other: U) -> Self::Output {
//         self *= other;
//         self
//     }
// }

// impl<T, U> MulAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: MulAssign<U>,
// {
//     fn mul_assign(&mut self, other: U) {
//         *self.value().lock() *= other;
//     }
// }

// impl<T, U> Rem<U> for State<T>
// where
//     T: Display + Clone + Rem<U> + RemAssign<U>,
// {
//     type Output = Self;

//     fn rem(mut self, other: U) -> Self::Output {
//         self %= other;
//         self
//     }
// }

// impl<T, U> RemAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: RemAssign<U>,
// {
//     fn rem_assign(&mut self, other: U) {
//         *self.value().lock() %= other;
//     }
// }

// impl<T, U> Sub<U> for State<T>
// where
//     T: Display + Clone + Sub<U> + SubAssign<U>,
// {
//     type Output = Self;

//     fn sub(mut self, other: U) -> Self::Output {
//         self -= other;
//         self
//     }
// }

// impl<T, U> SubAssign<U> for State<T>
// where
//     T: Display + Clone,
//     T: SubAssign<U>,
// {
//     fn sub_assign(&mut self, other: U) {
//         *self.value().lock() -= other;
//     }
// }
