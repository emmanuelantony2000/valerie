#![no_std]

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
pub use app::*;
use core::fmt::Display;
use html::Html;
use tree::{RcTreeNode, Tree};
pub use web_sys::{Element, Node};

pub mod app;
pub mod html;
mod tree;
// mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub trait Component {
//     fn view(&mut self) -> Element;
// }

pub trait Component {
    fn view(&self) -> (String, RcTreeNode);
}

impl<T> Component for T
where
    T: Display,
{
    fn view(&self) -> (String, RcTreeNode) {
        (self.to_string(), Tree::default().root())
    }
}

pub trait Page {
    fn view(&mut self) -> Html;
}
