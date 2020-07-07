use alloc::string::ToString;
use core::fmt::Display;

use wasm_bindgen::JsCast;

use crate::function::create_text_element;

/// `Component` trait
pub trait Component: Into<crate::Node> {}

impl<T> Component for T where T: Display {}

impl<T> From<T> for crate::Node
where
    T: Display,
{
    fn from(x: T) -> Self {
        Self(create_text_element(x.to_string()).unchecked_into())
    }
}
