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

use serde_json::ser::{CharEscape, CompactFormatter, Formatter};
use std::io;

use crate::error::Error;
use crate::options::Options;
use crate::token::Token;

fn write_to_vec<F: FnOnce(&mut Vec<u8>) -> io::Result<()>>(f: F) -> io::Result<Vec<u8>> {
    let mut vec = vec![];

    f(&mut vec).map(|()| vec)
}

macro_rules! write_func {
    ($name:ident ( )) => {
        fn $name<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
            let vec = write_to_vec(|v| CompactFormatter.$name(v))?;

            self.token.push(Token::Data(vec.into()));
            self.format_json(writer)
        }
    };

    ($name:ident ( $value:ty ) ) => {
        fn $name<W: ?Sized + io::Write>(
            &mut self,
            writer: &mut W,
            value: $value,
        ) -> io::Result<()> {
            let vec = write_to_vec(|v| CompactFormatter.$name(v, value))?;

            self.token.push(Token::Data(vec.into()));
            self.format_json(writer)
        }
    };
}

/// The formatter.
///
/// The `PrettyCompactFormatter` type implements a pretty [serde_json]
/// formatter which tries to be as compact as possible. This can increase the
/// readability of formatted JSON.
///
/// # Basic usage
///
/// * Use [`PrettyCompactFormatter::new()`] to create a formatter with a set of
///   default rules.
///
///   Defaults are:
///   - indentation: 2 characters
///   - maximum line length: 120 characters
///
/// * Use [`PrettyCompactFormatter::no_rules()`] without any rules.
///
///   Without any further applied configuration it acts as a plain pretty
///   formatter.
///
/// # Configuration
///
/// * Change the indentation with [`PrettyCompactFormatter::with_indent`].
///
/// ```
/// use json_pretty_compact::PrettyCompactFormatter;
///
/// let formatter = PrettyCompactFormatter::new().with_indent(4);
/// ```
///
/// * Change the maximum line length length with
///   [`PrettyCompactFormatter::with_max_line_length`].
///
/// ```
/// use json_pretty_compact::PrettyCompactFormatter;
///
/// let formatter = PrettyCompactFormatter::new().with_max_line_length(80);
/// ```
pub struct PrettyCompactFormatter {
    options: Options,
    token: Vec<Token>,
    level: u32,
}

impl PrettyCompactFormatter {
    /// Creates a `PrettyCompactFormatter` with a set of default rules.
    pub fn new() -> PrettyCompactFormatter {
        Self {
            options: Options::default(),
            token: vec![],
            level: 0,
        }
    }

    /// Creates a `PrettyCompactFormatter` without any rules applied.
    pub fn no_rules() -> PrettyCompactFormatter {
        Self {
            options: Options::no_rules(),
            ..Self::new()
        }
    }

    /// Changes the indentation to the given value.
    pub fn with_indent(mut self, indent: u32) -> Self {
        self.options.set_indent(indent);
        self
    }

    /// Changes the maximum line length to the given value.
    pub fn with_max_line_length(mut self, len: u32) -> Self {
        self.options.set_max_len(len);
        self
    }

    fn format_json<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.token.last().map_or(false, |t| t.is_end_array()) {
            self.reduce_array()?;
        } else if self.token.last().map_or(false, |t| t.is_end_object()) {
            self.reduce_object()?;
        }

        if self.token.len() == 1 {
            self.token[0].format(writer, &self.options)?;
        }

        Ok(())
    }

    fn reduce_array(&mut self) -> Result<(), Error> {
        let (idx, level) = self
            .find_last_token(|t| t.as_begin_array())
            .ok_or(Error::NoArrayStart)?;

        let mut array = self.token.drain(idx..).collect::<Vec<Token>>();

        array.remove(0);
        array.pop();

        self.token.push(Token::Array(level, array));

        Ok(())
    }

    fn reduce_object(&mut self) -> Result<(), Error> {
        let (idx, level) = self
            .find_last_token(|t| t.as_begin_object())
            .ok_or(Error::NoObjectStart)?;

        let mut object = self.token.drain(idx..).collect::<Vec<Token>>();

        object.remove(0);
        object.pop();

        self.token.push(Token::Object(level, object));

        Ok(())
    }

    fn find_last_token<P: FnMut(&Token) -> Option<u32>>(
        &self,
        mut predicate: P,
    ) -> Option<(usize, u32)> {
        self.token
            .iter()
            .enumerate()
            .rev()
            .find_map(|(idx, ev)| predicate(ev).map(|n| (idx, n)))
    }
}

impl Default for PrettyCompactFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for PrettyCompactFormatter {
    write_func!(write_null());
    write_func!(write_bool(bool));
    write_func!(write_i8(i8));
    write_func!(write_i16(i16));
    write_func!(write_i32(i32));
    write_func!(write_i64(i64));
    write_func!(write_i128(i128));
    write_func!(write_u8(u8));
    write_func!(write_u16(u16));
    write_func!(write_u32(u32));
    write_func!(write_u64(u64));
    write_func!(write_u128(u128));
    write_func!(write_f32(f32));
    write_func!(write_f64(f64));
    write_func!(write_number_str(&str));

    fn begin_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.token.push(Token::Data(b"\"".to_vec()));

        Ok(())
    }

    fn end_string<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let t = self.token.last_mut().ok_or(Error::EmptyTokenQueue)?;
        let data = t.as_data_mut_err()?;

        data.extend_from_slice(b"\"");

        self.format_json(writer)
    }

    fn write_string_fragment<W: ?Sized + io::Write>(
        &mut self,
        _writer: &mut W,
        fragment: &str,
    ) -> io::Result<()> {
        let t = self.token.last_mut().ok_or(Error::EmptyTokenQueue)?;
        let data = t.as_data_mut_err()?;

        data.extend_from_slice(fragment.as_bytes());

        Ok(())
    }

    fn write_char_escape<W: ?Sized + io::Write>(
        &mut self,
        _writer: &mut W,
        char_escape: CharEscape,
    ) -> io::Result<()> {
        let vec = write_to_vec(|v| CompactFormatter.write_char_escape(v, char_escape))?;

        let t = self.token.last_mut().ok_or(Error::EmptyTokenQueue)?;
        let data = t.as_data_mut_err()?;

        data.extend_from_slice(&vec);

        Ok(())
    }

    write_func!(write_byte_array(&[u8]));

    fn begin_array<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.token.push(Token::BeginArray(self.level));
        self.level += 1;

        Ok(())
    }

    fn end_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.token.push(Token::EndArray);
        self.level -= 1;

        self.format_json(writer)
    }

    fn begin_array_value<W: ?Sized + io::Write>(
        &mut self,
        _writer: &mut W,
        _first: bool,
    ) -> io::Result<()> {
        Ok(())
    }

    fn end_array_value<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn begin_object<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.token.push(Token::BeginObject(self.level));
        self.level += 1;

        Ok(())
    }

    fn end_object<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.token.push(Token::EndObject);
        self.level -= 1;

        self.format_json(writer)
    }

    fn begin_object_key<W: ?Sized + io::Write>(
        &mut self,
        _writer: &mut W,
        _first: bool,
    ) -> io::Result<()> {
        Ok(())
    }

    fn end_object_key<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn begin_object_value<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn end_object_value<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }

    write_func!(write_raw_fragment(&str));
}
