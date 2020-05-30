use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::convert::From;
use core::fmt::Display;
use core::ops::Deref;

#[derive(Clone)]
pub struct Channel {
    stream: Arc<String>,
}

impl<T> From<T> for Channel
where
    T: Display,
{
    fn from(x: T) -> Self {
        Self {
            stream: Arc::new(x.to_string()),
        }
    }
}

impl Deref for Channel {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}
