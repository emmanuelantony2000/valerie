use super::Tag;

use core::ops::{Deref, DerefMut};

pub struct Paragraph(Tag);

impl Paragraph {
    pub fn new() -> Self {
        Self(Tag::new("p"))
    }
}

impl Deref for Paragraph {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Paragraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Paragraph::new()
    }
}
