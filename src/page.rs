use crate::html::Tag;

pub trait Page {
    fn view(&mut self) -> Tag;
}
