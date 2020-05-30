use super::Tag;

use core::ops::{Deref, DerefMut};

pub struct Button(Tag);

impl Button {
    pub fn new() -> Self {
        Self(Tag::new("button"))
    }
}

impl Deref for Button {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Button {
    fn default() -> Self {
        Button::new()
    }
}
