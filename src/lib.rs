#![no_std]

extern crate alloc;

pub use app::*;
pub use channel::Channel;
pub use component::Component;
pub use function::{Event, Function, FunctionType};
pub use page::Page;
pub use state::{StateTrait, StateAtomic, StateMutex, StateVec};

pub mod app;
pub mod channel;
pub mod component;
pub mod function;
pub mod html;
pub mod page;
pub mod state;
pub mod macros;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
