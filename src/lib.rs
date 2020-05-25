#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use core::fmt::Display;
use html::{Function, Html};

pub use app::*;
pub use futures_intrusive::channel::shared::{
    state_broadcast_channel as channel, StateReceiver, StateSender,
};
pub use web_sys::{Element, Node};

pub mod app;
pub mod html;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type Tree = trees::Tree<Function>;

pub trait Component {
    fn view(&mut self) -> (String, Tree);
}

impl<T> Component for T
where
    T: Display,
{
    fn view(&mut self) -> (String, Tree) {
        (self.to_string(), Tree::new(Function::new()))
    }
}

pub trait Page {
    fn view(&mut self) -> Html;
}
