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

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Data(Vec<u8>);

impl Data {
    pub fn push_slice(&mut self, buf: &[u8]) {
        self.0.extend_from_slice(buf);
    }

    pub fn push_str(&mut self, fragment: &str) {
        self.push_slice(fragment.as_bytes());
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Data {
    fn from(vec: Vec<u8>) -> Data {
        Data(vec)
    }
}

#[derive(Debug)]
pub enum Token {
    BeginObject(u32),
    EndObject,
    BeginArray(u32),
    EndArray,
    Data(Data),
}

impl Token {
    pub fn as_begin_object(&self) -> Option<u32> {
        match self {
            Self::BeginObject(level) => Some(*level),
            _ => None,
        }
    }

    pub fn as_begin_object_err(&self) -> Result<u32, Error> {
        self.as_begin_object()
            .ok_or_else(|| Error::unexpected_event("BeginObject", self.debug_info()))
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

    pub fn as_begin_array_err(&self) -> Result<u32, Error> {
        self.as_begin_array()
            .ok_or_else(|| Error::unexpected_event("BeginArray", self.debug_info()))
    }

    pub fn is_end_array(&self) -> bool {
        matches!(self, Self::EndArray)
    }

    pub fn as_data(&self) -> Option<&Data> {
        match self {
            Self::Data(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_data_err(&self) -> Result<&Data, Error> {
        self.as_data()
            .ok_or_else(|| Error::unexpected_event("Data", self.debug_info()))
    }

    pub fn as_data_ref(&self) -> Option<&[u8]> {
        match self {
            Self::Data(data) => Some(data.as_ref()),
            _ => None,
        }
    }

    pub fn as_data_mut(&mut self) -> Option<&mut Data> {
        match self {
            Self::Data(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_data_mut_err(&mut self) -> Result<&mut Data, Error> {
        let di = self.debug_info();

        self.as_data_mut()
            .ok_or_else(|| Error::unexpected_event("Data", di))
    }

    fn debug_info(&self) -> &'static str {
        match self {
            Self::BeginObject(_) => "BeginObject",
            Self::EndObject => "EndObject",
            Self::BeginArray(_) => "BeginArray",
            Self::EndArray => "EndArray",
            Self::Data(_) => "Data",
        }
    }
}
