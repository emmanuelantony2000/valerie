pub mod app;
pub mod html;
mod utils;

use core::ops::Deref;
use web_sys::Node;
pub use app::*;

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub trait Component: Deref<Target = Node> {}
