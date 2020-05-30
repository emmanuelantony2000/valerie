#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use core::fmt::Display;
use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
use html::Html;

pub use app::*;
pub use channel::Channel;
pub use function::Function;
pub use state::{State, StateAtomic, StateMutex};
pub use web_sys::{Element, Node};

pub mod app;
pub mod channel;
pub mod function;
pub mod html;
pub mod state;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type Tree = trees::Tree<Function>;
pub type Receiver = StateReceiver<Channel>;
pub type Sender = StateSender<Channel>;

pub fn channel() -> (Sender, Receiver) {
    state_broadcast_channel::<Channel>()
}

pub trait Component {
    fn view(&mut self) -> (String, Tree);
}

impl<T> Component for T
where
    T: Display + Clone,
{
    fn view(&mut self) -> (String, Tree) {
        (self.to_string(), Tree::new(Function::new()))
    }
}

pub trait Page {
    fn view(&mut self) -> Html;
}
