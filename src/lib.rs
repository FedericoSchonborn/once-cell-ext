#![warn(clippy::pedantic, clippy::cargo)]

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OnceOption<T>(Option<T>);

impl<T> OnceOption<T> {
    /// # Errors
    ///
    /// This function fails if `self` is already occupied.
    pub fn set(&mut self, value: T) -> Result<(), SomeError> {
        self.set_or(value, SomeError)
    }

    /// # Errors
    ///
    /// This function returns `error` if `self` is already occupied.
    pub fn set_or<E>(&mut self, value: T, error: E) -> Result<(), E>
    where
        E: Error,
    {
        if self.0.is_some() {
            Err(error)
        } else {
            self.0 = Some(value);
            Ok(())
        }
    }

    /// # Errors
    ///
    /// This function returns `error()` if `self` is already occupied.
    pub fn set_or_else<E, F>(&mut self, value: T, error: F) -> Result<(), E>
    where
        E: Error,
        F: FnOnce() -> E,
    {
        if self.0.is_some() {
            Err(error())
        } else {
            self.0 = Some(value);
            Ok(())
        }
    }

    pub fn into_inner(self) -> Option<T> {
        self.0
    }

    pub const fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut()
    }

    pub const fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub const fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SomeError;

impl Error for SomeError {}

impl Display for SomeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        "attempt to write to a `Some` value".fmt(f)
    }
}
