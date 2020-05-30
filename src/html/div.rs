use super::Tag;

use core::ops::{Deref, DerefMut};

pub struct Div(Tag);

impl Div {
    pub fn new() -> Self {
        Self(Tag::new("div"))
    }
}

impl Deref for Div {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Div {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Div {
    fn default() -> Self {
        Div::new()
    }
}
