use alloc::string::ToString;
use core::fmt::Display;

use crate::function::document;

/// `Component` trait
pub trait Component {
    /// Return type of the view function
    type Type: AsRef<web_sys::Node>;

    fn view(self) -> Self::Type;
}

impl<T> Component for T
where
    T: Display,
{
    type Type = web_sys::Text;

    fn view(self) -> Self::Type {
        document().create_text_node(&self.to_string())
    }
}
