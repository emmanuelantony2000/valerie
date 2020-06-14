// use crate::Tree;
use crate::{Function, FunctionType};

use alloc::string::{ToString, String};
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

// pub trait Component {
//     fn view(&mut self) -> (String, Tree);
// }

// impl<T> Component for T
// where
//     T: Display + Clone,
// {
//     fn view(&mut self) -> (String, Tree) {
//         (self.to_string(), Tree::new(Function::new()))
//     }
// }