#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
pub use app::*;
use core::fmt::Display;
use html::{Html, Function};
use trees::Tree;
pub use web_sys::{Element, Node};

pub mod app;
pub mod html;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub trait Component {
    fn view(&mut self) -> (String, Tree<Function>);
}

impl<T> Component for T
where
    T: Display,
{
    fn view(&mut self) -> (String, Tree<Function>) {
        (self.to_string(), Tree::new(Function::new()))
    }
}

pub trait Page {
    fn view(&mut self) -> Html;
}
