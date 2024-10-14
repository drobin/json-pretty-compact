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

const DEFAULT_INDENT: u32 = 2;
const DEFAULT_MAX_LEN: Option<u32> = Some(120);

#[derive(Debug)]
pub struct Options {
    indent: u32,
    max_len: Option<u32>,
}

impl Options {
    pub fn no_rules() -> Options {
        Options {
            max_len: None,
            ..Self::default()
        }
    }

    pub fn indent(&self) -> u32 {
        self.indent
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.indent = indent
    }

    pub fn max_len(&self) -> Option<u32> {
        self.max_len
    }

    pub fn set_max_len(&mut self, max_len: u32) {
        self.max_len = Some(max_len);
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            indent: DEFAULT_INDENT,
            max_len: DEFAULT_MAX_LEN,
        }
    }
}
