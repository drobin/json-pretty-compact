// MIT License
//
// Copyright (c) 2024 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::io;
use thiserror::Error;

/// Error type of the library
#[derive(Debug, Error)]
pub enum Error {
    /// An unexpected event was detected.
    ///
    /// Tried to get an event of type `expected` but got `found`.
    #[error("unexpected event (expected {expected}, found {found})")]
    UnexpectedEvent { expected: String, found: String },

    /// The internal token-queue is empty.
    #[error("the event queue is empty")]
    EmptyTokenQueue,

    /// Could not find the array-start token.
    #[error("could not find start of array")]
    NoArrayStart,

    /// Could not find the object-start token.
    #[error("could not find start of object")]
    NoObjectStart,
}

impl Error {
    pub(crate) fn unexpected_event(expected: &str, found: &str) -> Error {
        Error::UnexpectedEvent {
            expected: expected.to_string(),
            found: found.to_string(),
        }
    }
}

impl From<Error> for io::Error {
    fn from(cause: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, cause)
    }
}
