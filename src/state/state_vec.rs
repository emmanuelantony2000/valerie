use crate::{Component, Function, FunctionType};
use super::{State, StateAtomic, StateMutex};

use crossbeam::atomic::AtomicCell;
use futures_intrusive::channel::StateId;
use futures_intrusive::channel::shared::{Sender, Receiver, unbuffered_channel, channel};
use core::fmt::Display;
use alloc::vec::Vec;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use alloc::sync::Arc;
use web_sys::{Node};
use parking_lot::RwLock;

// #[derive(Clone)]
// enum Change<T> where T: Copy + Display {
//     Add(usize, StateAtomic<T>),
//     AddLast(StateAtomic<T>),
//     Remove(usize),
// }

#[derive(Clone)]
enum Change<T> where T: State + Send {
    Add(usize, T),
    AddLast(T),
    Remove(usize),
}

struct SenderReceiver<T> where T: State + Send + 'static {
    tx: Sender<Change<T>>,
    // rx: Receiver<Change<T>>,
}

#[derive(Clone, Default)]
pub struct StateVec<T>
where
    T: State + Send + 'static,
{
    value: Arc<RwLock<Vec<T>>>,
    txrx: Arc<RwLock<Vec<Sender<Change<T>>>>>,
    // rx: Arc<Receiver<Change<T>>>,
}

// impl<T> StateVec<T> for StateVec<T>
// where
//     T: Display + Copy,
// {}

impl<T> StateVec<T>
where
    T: State + Send,
{
    pub fn new() -> Self {
        Self {
            value: Arc::new(RwLock::new(Vec::new())),
            txrx: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn from_atomic<U>(value: Vec<U>) -> StateVec<StateAtomic<U>> where U: Copy + Send + Display {
        // let (tx, rx) = unbuffered_channel();
        StateVec {
            value: Arc::new(RwLock::new(value.into_iter().map(StateAtomic::new).collect())),
            txrx: Arc::new(RwLock::new(Vec::new())),
            // rx: Arc::new(rx),
        }
    }

    pub fn from_mutex<U>(value: Vec<U>) -> StateVec<StateMutex<U>> where U: Clone + Send + Display {
        // let (tx, rx) = unbuffered_channel();
        StateVec {
            value: Arc::new(RwLock::new(value.into_iter().map(StateMutex::new).collect())),
            txrx: Arc::new(RwLock::new(Vec::new())),
            // rx: Arc::new(rx),
        }
    }

    pub fn view<F>(&self, encloser: Function, object: F) -> Function
    where
        F: FnOnce(T) -> Function,
        F: Clone + 'static,
    {
        for i in self.value.read().iter() {
            encloser.push_child(&object.clone()(i.clone()));
        }

        if let Some(element) = encloser.element() {
            // self.count.fetch_add(1);
            let (tx, rx) = unbuffered_channel();
            self.txrx.write().push(tx);//, rx: rx.clone()});
            // let rx = self.rx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                while let Some(change) = rx.receive().await {
                    match change {
                        Change::Add(i, x) => {

                        },
                        Change::AddLast(x) => {
                            element.append_child(&object.clone()(x)).unwrap();
                        }
                        Change::Remove(i) => {
                            element.remove_child(&element.children().item(i as u32).unwrap()).unwrap();
                        },
                    }
                }
            });
        }

        encloser
    }

    // pub fn push(&self, value: T) {
    //     self.value.write().push(StateAtomic::new(value));
    //     self.update(Change::AddLast(StateAtomic::new(value)));
    // }

    pub fn remove(&self, index: usize) {
        self.value.write().remove(index);
        self.update(Change::Remove(index));
    }

    pub fn len(&self) -> usize {
        self.value.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.read().is_empty()
    }

    fn update(&self, change: Change<T>) {
        self.txrx.read().iter().for_each(|tx| {
            let tx = tx.clone();
            let change = change.clone();
            wasm_bindgen_futures::spawn_local(async move {
                tx.send(change).await.unwrap_or(());
            });
        });
    }
}

impl<T> StateVec<StateAtomic<T>> where T: Copy + Send + Display {
    pub fn push(&self, value: T) {
        self.value.write().push(StateAtomic::new(value));
        self.update(Change::AddLast(StateAtomic::new(value)));
    }
}

impl<T> StateVec<StateMutex<T>> where T: Clone + Send + Display {
    pub fn push(&self, value: T) {
        self.value.write().push(StateMutex::new(value.clone()));
        self.update(Change::AddLast(StateMutex::new(value)));
    }
}

// impl<T> Component for StateVec<T> where T: Display + Copy {
//     fn view(self) -> Function {
//         let wrap = Function::new(FunctionType::Element("div"));
//         for i in self.value.read().iter() {
//             wrap.push_child(&i.view())
//         }
//         // function.rx = Some(self.rx());
//         wasm_bindgen_futures::spawn_local(change(wrap.clone(), self.rx()));
//         // let node = function

//         wrap
//     }
// }

// pub async fn change(node: Node, rx: Arc<Receiver<Change>>) {
//     let mut old = StateId::new();
//     while let Some((new, value)) = rx.receive(old).await {
//         node.set_node_value(Some(&value));
//         old = new;
//     }
// }

// impl<T, U> Add<U> for StateVec<T>
// where
//     T: Display + Copy + Add<U> + AddAssign<U>,
// {
//     type Output = Self;

//     fn add(mut self, other: U) -> Self::Output {
//         self += other;
//         self
//     }
// }

// impl<T, U> AddAssign<U> for StateVec<T>
// where
//     T: Display + Copy + AddAssign<U>,
// {
//     fn add_assign(&mut self, other: U) {
//         let mut value = self.value();
//         value += other;
//         self.put(value);
//     }
// }

// impl<T, U> Div<U> for StateVec<T>
// where
//     T: Display + Copy + Div<U> + DivAssign<U>,
// {
//     type Output = Self;

//     fn div(mut self, other: U) -> Self::Output {
//         self /= other;
//         self
//     }
// }

// impl<T, U> DivAssign<U> for StateVec<T>
// where
//     T: Display + Copy + DivAssign<U>,
// {
//     fn div_assign(&mut self, other: U) {
//         let mut value = self.value();
//         value /= other;
//         self.put(value);
//     }
// }

// impl<T, U> Mul<U> for StateVec<T>
// where
//     T: Display + Copy + Mul<U> + MulAssign<U>,
// {
//     type Output = Self;

//     fn mul(mut self, other: U) -> Self::Output {
//         self *= other;
//         self
//     }
// }

// impl<T, U> MulAssign<U> for StateVec<T>
// where
//     T: Display + Copy + MulAssign<U>,
// {
//     fn mul_assign(&mut self, other: U) {
//         let mut value = self.value();
//         value *= other;
//         self.put(value);
//     }
// }

// impl<T, U> Rem<U> for StateVec<T>
// where
//     T: Display + Copy + Rem<U> + RemAssign<U>,
// {
//     type Output = Self;

//     fn rem(mut self, other: U) -> Self::Output {
//         self %= other;
//         self
//     }
// }

// impl<T, U> RemAssign<U> for StateVec<T>
// where
//     T: Display + Copy + RemAssign<U>,
// {
//     fn rem_assign(&mut self, other: U) {
//         let mut value = self.value();
//         value %= other;
//         self.put(value);
//     }
// }

// impl<T, U> Sub<U> for StateVec<T>
// where
//     T: Display + Copy + Sub<U> + SubAssign<U>,
// {
//     type Output = Self;

//     fn sub(mut self, other: U) -> Self::Output {
//         self -= other;
//         self
//     }
// }

// impl<T, U> SubAssign<U> for StateVec<T>
// where
//     T: Display + Copy + SubAssign<U>,
// {
//     fn sub_assign(&mut self, other: U) {
//         let mut value = self.value();
//         value -= other;
//         self.put(value);
//     }
// }
