//! The `Messages` newtype provides an ergonomic struct for storing and printing messages.
//! 
//! (Also, I should note that this kind of documentation is not much use in a binary crate like 
//! this one. If this were a library crate, the code below would be tested and documentation could
//! be generated based on it. That won't happen here, but I thought I'd include the docs anyway.)
//! 
//! # Rationale
//! 
//! Onera has been using a bare vector to store messages, but this is not exactly what I'd call 
//! easy. Part of the problem can be addressed by using a Copy-On-Write wrapper around
//! `&'static str` and `String`, but doing that still leaves you writing `Cow::from("ugh")` when
//! you could just be writing `"ugh"`. Who wants that? This wrapper provides an interface to the 
//! underlying vector which does that annoying work for you.
//! 
//! ## Examples
//! 
//! ```Rust
//! let mut messages = Messages::new();
//! 
//! messages.push("Hello, world!");
//! messages.push("Note that static string slices...");
//! 
//! let message_string = String::from("...Work the same as owned strings.");
//! messages.push(message_string);
//! 
//! println!("{:?}", messages.get());
//! messages.clear();
//! ```
//! 
//! You might also notice the use of `clear()` above. This method has been added to permit the 
//! removal of items from the vector *without* deallocating the vector so that memory does not 
//! need to be allocated again.

use std::borrow::Cow;

#[derive(Default)]
pub struct Messages(Vec<Cow<'static, str>>);

impl Messages {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a message.
    pub fn push<T: Into<Cow<'static, str>>>(&mut self, message: T) {
        self.0.push(message.into())
    }

    /// Returns a reference to stored messages.
    pub fn get(&self) -> &[Cow<'static, str>] {
        &self.0
    }

    /// Clears existing messages without reallocating the internal vector.
    /// 
    /// This allows us to avoid reallocating each time we want to clear out the messages.
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
