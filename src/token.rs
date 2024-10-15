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

#[cfg(test)]
mod tests;

use std::fmt::{self, Display, Formatter};
use std::io;

use crate::error::Error;
use crate::options::Options;

macro_rules! write_indent {
    ($writer:expr, $len:ident) => {
        if $len > 0 {
            write!($writer, "{:len$}", " ", len = $len)?;
        }
    };
}

#[derive(Debug)]
pub enum Token {
    BeginObject(u32),
    EndObject,
    BeginArray(u32),
    EndArray,
    Data(Vec<u8>),
    Array(u32, Vec<Token>),
    Object(u32, Vec<Token>),
}

impl Token {
    pub fn as_begin_object(&self) -> Option<u32> {
        match self {
            Self::BeginObject(level) => Some(*level),
            _ => None,
        }
    }

    pub fn is_end_object(&self) -> bool {
        matches!(self, Self::EndObject)
    }

    pub fn as_begin_array(&self) -> Option<u32> {
        match self {
            Self::BeginArray(level) => Some(*level),
            _ => None,
        }
    }

    pub fn is_end_array(&self) -> bool {
        matches!(self, Self::EndArray)
    }

    pub fn as_data(&self) -> Option<&[u8]> {
        match self {
            Self::Data(data) => Some(data.as_ref()),
            _ => None,
        }
    }

    pub fn as_data_err(&self) -> Result<&[u8], Error> {
        self.as_data()
            .ok_or_else(|| Error::unexpected_event("Data", self.debug_info()))
    }

    pub fn as_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        match self {
            Self::Data(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_data_mut_err(&mut self) -> Result<&mut Vec<u8>, Error> {
        let di = self.debug_info();

        self.as_data_mut()
            .ok_or_else(|| Error::unexpected_event("Data", di))
    }

    pub fn length(&self) -> usize {
        match self {
            Token::BeginObject(_) | Token::EndObject | Token::BeginArray(_) | Token::EndArray => 0,
            Token::Data(vec) => vec.len(),
            Token::Array(_, token) => {
                let n = token.iter().fold(0, |acc, t| acc + t.length());

                // add all commas between elements
                let inner = n + (token.len().checked_sub(1).unwrap_or(0) * 2);

                if inner > 0 {
                    4 + inner // plus surrounding [ ]
                } else {
                    3 // [ ]
                }
            }
            Token::Object(_, token) => {
                let n = token.iter().fold(0, |acc, t| acc + t.length());
                let num_keys = token.len() / 2;

                // add ": " between key & value and commas between elements
                let inner = n + 2 * num_keys + (num_keys.checked_sub(1).unwrap_or(0) * 2);

                if inner > 0 {
                    4 + inner // plus surrounding {}
                } else {
                    3 // [ ] or { }
                }
            }
        }
    }

    pub fn format<W: ?Sized + io::Write>(
        &self,
        writer: &mut W,
        options: &Options,
        forced_compact: Option<bool>,
    ) -> io::Result<()> {
        match self {
            Token::BeginObject(_) | Token::EndObject | Token::BeginArray(_) | Token::EndArray => {}
            Token::Data(vec) => writer.write_all(vec)?,
            Token::Array(level, token) => {
                let compact = forced_compact.unwrap_or_else(|| self.can_compact(options, None));
                let mut first = true;

                let spaces = (level * options.indent()) as usize;
                let spaces_next = ((level + 1) * options.indent()) as usize;

                if compact {
                    writer.write_all(b"[ ")?;
                } else {
                    writer.write_all(b"[\n")?;
                }

                for t in token {
                    if !first {
                        if compact {
                            writer.write_all(b", ")?;
                        } else {
                            writer.write_all(b",\n")?;
                        }
                    }

                    if !compact {
                        write_indent!(writer, spaces_next);
                    }

                    t.format(writer, options, None)?;

                    first = false;
                }

                if compact && first {
                    writer.write_all(b"]")?;
                } else if compact && !first {
                    writer.write_all(b" ]")?;
                } else {
                    writer.write_all(b"\n")?;
                    write_indent!(writer, spaces);
                    writer.write_all(b"]")?;
                }
            }
            Token::Object(level, token) => {
                let compact = forced_compact.unwrap_or_else(|| self.can_compact(options, None));
                let mut first = true;

                let spaces = (level * options.indent()) as usize;
                let spaces_next = ((level + 1) * options.indent()) as usize;
                let mut cur_indent = 0;

                if compact {
                    writer.write_all(b"{ ")?;
                    cur_indent += 2;
                } else {
                    writer.write_all(b"{\n")?;
                    cur_indent = spaces;
                }

                let iter = token.chunks_exact(2).map(|chunk| (&chunk[0], &chunk[1]));

                for (t1, t2) in iter {
                    let key = t1.as_data_err()?;

                    if !first {
                        if compact {
                            writer.write_all(b", ")?;
                            cur_indent += 2;
                        } else {
                            writer.write_all(b",\n")?;
                            cur_indent = 0;
                        }
                    }

                    if !compact {
                        write_indent!(writer, spaces_next);
                        cur_indent += spaces_next;
                    }

                    writer.write_all(key)?;
                    writer.write_all(b": ")?;
                    cur_indent += key.len() + 2;

                    // Let's check if the value can be put compacted behind the key in one line.
                    let forced_compact = t2.can_compact(options, Some(cur_indent));

                    if !forced_compact {
                        // There is not enough space to put the value into the same line.
                        t2.format(writer, options, Some(false))?;
                    } else {
                        t2.format(writer, options, None)?;
                    }

                    first = false;
                }

                if compact && first {
                    writer.write_all(b"}")?;
                } else if compact && !first {
                    writer.write_all(b" }")?;
                } else {
                    writer.write_all(b"\n")?;
                    write_indent!(writer, spaces);
                    writer.write_all(b"}")?;
                }
            }
        };

        Ok(())
    }

    fn can_compact(&self, options: &Options, forced_indent: Option<usize>) -> bool {
        match self {
            Token::BeginObject(_)
            | Token::EndObject
            | Token::BeginArray(_)
            | Token::EndArray
            | Token::Data(_) => true,
            Token::Array(level, _) | Token::Object(level, _) => {
                options.max_len().is_some_and(|max| {
                    let prefix =
                        forced_indent.unwrap_or_else(|| (level * options.indent()) as usize);

                    prefix + self.length() < max as usize
                })
            }
        }
    }

    fn debug_info(&self) -> &'static str {
        match self {
            Self::BeginObject(_) => "BeginObject",
            Self::EndObject => "EndObject",
            Self::BeginArray(_) => "BeginArray",
            Self::EndArray => "EndArray",
            Self::Data(_) => "Data",
            Self::Array(_, _) => "Array",
            Self::Object(_, _) => "Object",
        }
    }
}

impl Display for Token {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Token::BeginObject(_) => Ok(()),
            Token::EndObject => Ok(()),
            Token::BeginArray(_) => Ok(()),
            Token::EndArray => Ok(()),
            Token::Data(vec) => {
                write!(fmt, "{}", String::from_utf8_lossy(vec))
            }
            Token::Array(_, token) => {
                let vec = token.iter().map(|t| t.to_string()).collect::<Vec<_>>();

                write!(fmt, "[ {} ]", vec.join(", "))
            }
            Token::Object(_, token) => {
                let vec = token
                    .chunks_exact(2)
                    .map(|c| format!("{}: {}", c[0].to_string(), c[1].to_string()))
                    .collect::<Vec<_>>();

                write!(fmt, "{{ {} }}", vec.join(", "))
            }
        }
    }
}
