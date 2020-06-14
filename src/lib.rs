#![no_std]

extern crate alloc;

pub use app::*;
pub use channel::Channel;
pub use function::{Event, Function, FunctionType};
pub use state::{State, StateAtomic, StateMutex, StateVec};
pub use page::Page;
pub use component::Component;
// use tree::Tree;

pub mod app;
pub mod channel;
pub mod function;
pub mod html;
pub mod state;
pub mod tree;
pub mod component;
pub mod page;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub type Tree = trees::Tree<Function>;
