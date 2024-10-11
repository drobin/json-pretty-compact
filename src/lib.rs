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

//! A pretty formatter for [serde_json].
//!
//!
//! The `json_pretty_compact` crate implements a pretty [serde_json] formatter
//! which tries to be as compact as possible. This can increase the readability
//! of formatted JSON. Look at the following comparison.
//!
//! * A pure pretty formatted JSON looks like this:
//!
//! ```json
//! [
//!   {
//!     "name": {
//!       "first": "Kobe",
//!       "middle": "Nico",
//!       "last": "Grimes"
//!     },
//!     "phoneNumber": "1-547-479-5471 x062",
//!     "username": "Kobe-Grimes",
//!     "emails": [
//!       "Melyssa.Cremin4@gmail.com",
//!       "Jayne.Green37@gmail.com"
//!     ],
//!     "coordinates": {
//!       "latitude": "-66.3821",
//!       "longitude": "127.117"
//!     }
//!   },
//!   {
//!     "name": {
//!       "first": "Adrian",
//!       "middle": "Finley",
//!       "last": "Koch"
//!     },
//!     "phoneNumber": "1-420-853-5251 x68083",
//!     "username": "Adrian-Koch",
//!     "emails": [
//!       "Andy99@gmail.com",
//!       "Elenor.Aufderhar96@gmail.com"
//!     ],
//!     "coordinates": {
//!       "latitude": "51.4003",
//!       "longitude": "3.351"
//!     }
//!   }
//! ]
//! ```
//!
//! * Where the same JSON in a pretty compact format looks like this:
//!
//! ```json
//! [
//!   {
//!     "name": { "first": "Kobe", "middle": "Nico", "last": "Grimes" },
//!     "phoneNumber": "1-547-479-5471 x062",
//!     "username": "Kobe-Grimes",
//!     "emails": [ "Melyssa.Cremin4@gmail.com", "Jayne.Green37@gmail.com" ],
//!     "coordinates": { "latitude": "-66.3821", "longitude": "127.117" }
//!   },
//!   {
//!     "name": { "first": "Adrian", "middle": "Finley", "last": "Koch" },
//!     "phoneNumber": "1-420-853-5251 x68083",
//!     "username": "Adrian-Koch",
//!     "emails": [ "Andy99@gmail.com", "Elenor.Aufderhar96@gmail.com" ],
//!     "coordinates": { "latitude": "51.4003", "longitude": "3.351" }
//!   }
//! ]
//! ```
//!
//! ## Compaction rules
//!
//! The formatter tries to put arrays and objects into one line, as long as the
//! line length is still within its limit. If a line will become too large, the
//! formatter will change into a pretty format.
//!
//! Check the [`PrettyCompactFormatter`] documentation to find out how to
//! configure the formatter.
//!
//! ## Usage
//!
//! ```
//! use json_pretty_compact::PrettyCompactFormatter;
//! use serde::Serialize;
//! use serde_json::{Serializer, Value};
//!
//! // Create a JSON value.
//! // In this simple example it contains only the "true" value.
//! let value: Value = serde_json::from_str("true").unwrap();
//!
//! // The buffer where the serialized JSON is written.
//! let mut target = vec![];
//!
//! // Create the formatter.
//! // It takes all the default values.
//! let formatter = PrettyCompactFormatter::new();
//!
//! // Serialize the JSON value into the `target` buffer.
//! let mut ser = Serializer::with_formatter(&mut target, formatter);
//! value.serialize(&mut ser).unwrap();
//!
//! assert_eq!(target, b"true");
//! ```
//!
//!
//! [serde_json]: https://docs.rs/serde_json/latest/serde_json/index.html

mod error;
mod fmt;
mod token;

pub use crate::error::Error;
pub use crate::fmt::PrettyCompactFormatter;
