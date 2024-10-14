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
use std::io::{self, Cursor, Write};

use crate::error::Error;
use crate::token::Token;

const DEFAULT_INDENT: u32 = 2;
const DEFAULT_MAX_LEN: Option<u32> = Some(120);

fn write_to_vec<F: FnOnce(&mut Cursor<Vec<u8>>) -> io::Result<()>>(f: F) -> io::Result<Vec<u8>> {
    let mut cursor = Cursor::new(vec![]);

    f(&mut cursor).map(|()| cursor.into_inner())
}

macro_rules! write_indent {
    ($writer:expr, $len:ident) => {
        if $len > 0 {
            write!($writer, "{:len$}", " ", len = $len)?;
        }
    };
}

macro_rules! write_func {
    ($name:ident ( )) => {
        fn $name<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
            let vec = write_to_vec(|cursor| CompactFormatter.$name(cursor))?;

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
            let vec = write_to_vec(|cursor| CompactFormatter.$name(cursor, value))?;

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
    indent: u32,
    max_len: Option<u32>,
    token: Vec<Token>,
    level: u32,
}

impl PrettyCompactFormatter {
    /// Creates a `PrettyCompactFormatter` with a set of default rules.
    pub fn new() -> PrettyCompactFormatter {
        Self {
            max_len: DEFAULT_MAX_LEN,
            ..Self::no_rules()
        }
    }

    /// Creates a `PrettyCompactFormatter` without any rules applied.
    pub fn no_rules() -> PrettyCompactFormatter {
        Self {
            indent: DEFAULT_INDENT,
            max_len: None,
            token: vec![],
            level: 0,
        }
    }

    /// Changes the indentation to the given value.
    pub fn with_indent(mut self, indent: u32) -> Self {
        self.indent = indent;
        self
    }

    /// Changes the maximum line length to the given value.
    pub fn with_max_line_length(mut self, len: u32) -> Self {
        self.max_len = Some(len);
        self
    }

    fn format_json<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.token.last().map_or(false, |t| t.is_end_array()) {
            self.format_array()?;
        } else if self.token.last().map_or(false, |t| t.is_end_object()) {
            self.format_object()?;
        }

        if self.token.len() == 1 {
            if let Some(buf) = self.token[0].as_data() {
                writer.write_all(buf)?;
                self.token.pop();
            }
        }

        Ok(())
    }

    fn format_array(&mut self) -> io::Result<()> {
        let (idx, level) = self
            .find_last_token(|t| t.as_begin_array())
            .ok_or(Error::NoArrayStart)?;

        let compact = self.can_compact_array(idx)?;

        let mut cursor = Cursor::new(vec![]);
        let mut first = true;

        let spaces = (level * self.indent) as usize;
        let spaces_next = ((level + 1) * self.indent) as usize;

        if compact {
            cursor.write_all(b"[ ")?;
        } else {
            cursor.write_all(b"[\n")?;
        }

        for t in &self.token[idx + 1..self.token.len() - 1] {
            let value = t.as_data_err()?;

            if !first {
                if compact {
                    cursor.write_all(b", ")?;
                } else {
                    cursor.write_all(b",\n")?;
                }
            }

            if !compact {
                write_indent!(cursor, spaces_next);
            }

            cursor.write_all(value)?;

            first = false;
        }

        if compact && first {
            cursor.write_all(b"]")?;
        } else if compact && !first {
            cursor.write_all(b" ]")?;
        } else {
            cursor.write_all(b"\n")?;
            write_indent!(cursor, spaces);
            cursor.write_all(b"]")?;
        }

        self.token.drain(idx..);
        self.token.push(Token::Data(cursor.into_inner()));

        Ok(())
    }

    fn format_object(&mut self) -> io::Result<()> {
        let (idx, level) = self
            .find_last_token(|t| t.as_begin_object())
            .ok_or(Error::NoObjectStart)?;

        let compact = self.can_compact_object(idx)?;

        let mut cursor = Cursor::new(vec![]);
        let mut first = true;

        let spaces = (level * self.indent) as usize;
        let spaces_next = ((level + 1) * self.indent) as usize;

        if compact {
            cursor.write_all(b"{ ")?;
        } else {
            cursor.write_all(b"{\n")?;
        }

        let iter = self.token[idx + 1..]
            .chunks_exact(2)
            .map(|chunk| (&chunk[0], &chunk[1]));

        for (t1, t2) in iter {
            let key = t1.as_data_err()?;
            let value = t2.as_data_err()?;

            if !first {
                if compact {
                    cursor.write_all(b", ")?;
                } else {
                    cursor.write_all(b",\n")?;
                }
            }

            if !compact {
                write_indent!(cursor, spaces_next);
            }

            cursor.write_all(key)?;
            cursor.write_all(b": ")?;
            cursor.write_all(value)?;

            first = false;
        }

        if compact && first {
            cursor.write_all(b"}")?;
        } else if compact && !first {
            cursor.write_all(b" }")?;
        } else {
            cursor.write_all(b"\n")?;
            write_indent!(cursor, spaces);
            cursor.write_all(b"}")?;
        }

        self.token.drain(idx..);
        self.token.push(Token::Data(cursor.into_inner()));

        Ok(())
    }

    fn can_compact_array(&self, idx: usize) -> Result<bool, Error> {
        let level = self.token[idx].as_begin_array_err()?;
        let token = &self.token[idx + 1..self.token.len() - 1];

        if let Some(max_len) = self.max_len {
            let mut len = if token.is_empty() {
                3 // "[ ]"
            } else {
                let n = token
                    .iter()
                    .filter_map(|t| t.as_data())
                    .fold(0, |acc, buf| acc + buf.len());

                // add all the commas and spaces
                4 + n + (token.len() - 1) * 2
            };

            len += (level * self.indent) as usize;

            if len <= max_len as usize {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn can_compact_object(&self, idx: usize) -> Result<bool, Error> {
        let level = self.token[idx].as_begin_object_err()?;
        let token = &self.token[idx + 1..self.token.len() - 1];

        if let Some(max_len) = self.max_len {
            let mut len = if token.is_empty() {
                3 // { }
            } else {
                let n = token
                    .iter()
                    .filter_map(|t| t.as_data())
                    .fold(0, |acc, buf| acc + buf.len());

                // add all the commas and spaces
                4 + n + (token.len() - 1) * 3
            };

            len += (level * self.indent) as usize;

            if len <= max_len as usize {
                return Ok(true);
            }
        }

        Ok(false)
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
        let vec = write_to_vec(|cursor| CompactFormatter.write_char_escape(cursor, char_escape))?;

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
