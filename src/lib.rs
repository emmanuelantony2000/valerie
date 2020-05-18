#![no_std]

extern crate alloc;

pub use app::*;
pub use web_sys::{Element, Node};

pub mod app;
pub mod html;
// mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub trait Component {
    fn view(&mut self) -> Element;
}
