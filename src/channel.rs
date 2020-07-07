use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::convert::From;
use core::fmt::Display;
use core::ops::Deref;

/// A wrapper around `Arc<String>`
///
/// The format of the message that is passed from `Sender` to `Receiver`.
///
/// Rather than passing the value inside the state as it is and converting it to string in the
/// destination, the value is converted to the `Channel` type and then passed on to the receivers.
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
