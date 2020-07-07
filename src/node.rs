use core::ops::{Deref, DerefMut};

/// A wrapper for `web_sys::Node`
#[derive(Clone)]
pub struct Node(pub(crate) web_sys::Node);

impl Deref for Node {
    type Target = web_sys::Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Node> for web_sys::Node {
    fn from(x: Node) -> web_sys::Node {
        x.0
    }
}

impl AsRef<web_sys::Node> for Node {
    fn as_ref(&self) -> &web_sys::Node {
        &self.0
    }
}
