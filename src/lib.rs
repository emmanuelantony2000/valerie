#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
pub use app::*;
use core::ops::Deref;
pub use web_sys::{Element, Node};

pub mod app;
pub mod html;
// mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub trait Component: Deref<Target = Node> + Into<Element> {}

pub trait Component {
    fn view(&mut self) -> Element;
}


// pub struct State<T> {
//     value: Arc<T>,
//     change: Vec<(&'static str, Box<dyn FnMut()>)>,
// }

// impl<T> State<T> {
//     pub fn new(value: T) -> Self {
//         Self {
//             value: Arc::new(value),
//             change: Vec::new(),
//         }
//     }

//     pub fn push(&mut self, component: impl Component, function: Box<dyn FnMut()>, change: Change) -> impl Component
//     {
//         match change {
//             Change::Function(x) => {
//                 component.set_id("hello");
//                 self.change.push(("hello", function));
//             }
//         }

//         component
//     }
// }

// pub enum Change {
//     Function(Box<dyn FnMut()>),
// }
