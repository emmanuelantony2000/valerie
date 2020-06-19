use crate::{Function, FunctionType};

use alloc::string::ToString;
use core::fmt::Display;

pub trait Component {
    fn view(self) -> Function;
}

impl<T> Component for T
where
    T: Display + Clone,
{
    fn view(self) -> Function {
        Function::new(FunctionType::Text(self.to_string()))
    }
}
