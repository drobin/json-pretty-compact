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

use crate::error::Error;
use crate::token::{Data, Token};

#[test]
fn data_push_slice() {
    let mut data: Data = vec![].into();

    data.push_slice(b"");
    assert!(data.0.is_empty());

    data.push_slice(b"abc");
    assert_eq!(data.0, b"abc");

    data.push_slice(b"123");
    assert_eq!(data.0, b"abc123");
}

#[test]
fn data_push_str() {
    let mut data: Data = vec![].into();

    data.push_str("");
    assert!(data.0.is_empty());

    data.push_str("abc");
    assert_eq!(data.0, b"abc");

    data.push_str("123");
    assert_eq!(data.0, b"abc123");
}

#[test]
fn as_begin_object_begin_object() {
    let t = Token::BeginObject(4711);

    assert_eq!(t.as_begin_object().unwrap(), 4711);
}

#[test]
fn as_begin_object_end_object() {
    let t = Token::EndObject;

    assert!(t.as_begin_object().is_none());
}

#[test]
fn as_begin_object_begin_array() {
    let t = Token::BeginArray(4711);

    assert!(t.as_begin_object().is_none());
}

#[test]
fn as_begin_object_end_array() {
    let t = Token::EndArray;

    assert!(t.as_begin_object().is_none());
}

#[test]
fn as_begin_object_data() {
    let t = Token::Data(vec![].into());

    assert!(t.as_begin_object().is_none());
}

#[test]
fn as_begin_object_err_begin_object() {
    let t = Token::BeginObject(4711);

    assert_eq!(t.as_begin_object_err().unwrap(), 4711);
}

#[test]
fn as_begin_object_err_end_object() {
    let t = Token::EndObject;
    let err = t.as_begin_object_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginObject" && found == "EndObject"));
}

#[test]
fn as_begin_object_err_begin_array() {
    let t = Token::BeginArray(4711);
    let err = t.as_begin_object_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginObject" && found == "BeginArray"));
}

#[test]
fn as_begin_object_err_end_array() {
    let t = Token::EndArray;
    let err = t.as_begin_object_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginObject" && found == "EndArray"));
}

#[test]
fn as_begin_object_err_data() {
    let t = Token::Data(vec![].into());
    let err = t.as_begin_object_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginObject" && found == "Data"));
}

#[test]
fn is_end_object_begin_object() {
    let t = Token::BeginObject(4711);

    assert!(!t.is_end_object());
}

#[test]
fn is_end_object_end_object() {
    let t = Token::EndObject;

    assert!(t.is_end_object());
}

#[test]
fn is_end_object_begin_array() {
    let t = Token::BeginArray(4711);

    assert!(!t.is_end_object());
}

#[test]
fn is_end_object_end_array() {
    let t = Token::EndArray;

    assert!(!t.is_end_object());
}

#[test]
fn is_end_object_data() {
    let t = Token::Data(vec![].into());

    assert!(!t.is_end_object());
}

#[test]
fn as_begin_array_begin_object() {
    let t = Token::BeginObject(4711);

    assert!(t.as_begin_array().is_none());
}

#[test]
fn as_begin_array_end_object() {
    let t = Token::EndObject;

    assert!(t.as_begin_array().is_none());
}

#[test]
fn as_begin_array_begin_array() {
    let t = Token::BeginArray(4711);

    assert_eq!(t.as_begin_array().unwrap(), 4711);
}

#[test]
fn as_begin_array_end_array() {
    let t = Token::EndArray;

    assert!(t.as_begin_array().is_none());
}

#[test]
fn as_begin_array_data() {
    let t = Token::Data(vec![].into());

    assert!(t.as_begin_array().is_none());
}

#[test]
fn as_begin_array_err_begin_object() {
    let t = Token::BeginObject(4711);
    let err = t.as_begin_array_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginArray" && found == "BeginObject"));
}

#[test]
fn as_begin_array_err_end_object() {
    let t = Token::EndObject;
    let err = t.as_begin_array_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginArray" && found == "EndObject"));
}

#[test]
fn as_begin_array_err_begin_array() {
    let t = Token::BeginArray(4711);

    assert_eq!(t.as_begin_array_err().unwrap(), 4711);
}

#[test]
fn as_begin_array_err_end_array() {
    let t = Token::EndArray;
    let err = t.as_begin_array_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginArray" && found == "EndArray"));
}

#[test]
fn as_begin_array_err_data() {
    let t = Token::Data(vec![].into());
    let err = t.as_begin_array_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "BeginArray" && found == "Data"));
}

#[test]
fn is_end_array_begin_object() {
    let t = Token::BeginObject(4711);

    assert!(!t.is_end_array());
}

#[test]
fn is_end_array_end_object() {
    let t = Token::EndObject;

    assert!(!t.is_end_array());
}

#[test]
fn is_end_array_begin_array() {
    let t = Token::BeginArray(4711);

    assert!(!t.is_end_array());
}

#[test]
fn is_end_array_end_array() {
    let t = Token::EndArray;

    assert!(t.is_end_array());
}

#[test]
fn is_end_array_data() {
    let t = Token::Data(vec![].into());

    assert!(!t.is_end_array());
}

#[test]
fn as_data_begin_object() {
    let t = Token::BeginObject(4711);

    assert!(t.as_data().is_none());
}

#[test]
fn as_data_end_object() {
    let t = Token::EndObject;

    assert!(t.as_data().is_none());
}

#[test]
fn as_data_begin_array() {
    let t = Token::BeginArray(4711);

    assert!(t.as_data().is_none());
}

#[test]
fn as_data_end_array() {
    let t = Token::EndArray;

    assert!(t.as_data().is_none());
}

#[test]
fn as_data_data() {
    let t = Token::Data(vec![].into());

    assert_eq!(t.as_data().unwrap(), &vec![].into());
}

#[test]
fn as_data_err_begin_object() {
    let t = Token::BeginObject(4711);
    let err = t.as_data_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "BeginObject"));
}

#[test]
fn as_data_err_end_object() {
    let t = Token::EndObject;
    let err = t.as_data_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "EndObject"));
}

#[test]
fn as_data_err_begin_array() {
    let t = Token::BeginArray(4711);
    let err = t.as_data_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "BeginArray"));
}

#[test]
fn as_data_err_end_array() {
    let t = Token::EndArray;
    let err = t.as_data_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "EndArray"));
}

#[test]
fn as_data_err_data() {
    let t = Token::Data(vec![].into());

    assert_eq!(t.as_data_err().unwrap(), &vec![].into());
}

#[test]
fn as_data_ref_begin_object() {
    let t = Token::BeginObject(4711);

    assert!(t.as_data_ref().is_none());
}

#[test]
fn as_data_ref_end_object() {
    let t = Token::EndObject;

    assert!(t.as_data_ref().is_none());
}

#[test]
fn as_data_ref_begin_array() {
    let t = Token::BeginArray(4711);

    assert!(t.as_data_ref().is_none());
}

#[test]
fn as_data_ref_end_array() {
    let t = Token::EndArray;

    assert!(t.as_data_ref().is_none());
}

#[test]
fn as_data_mut_ref_data() {
    let t = Token::Data(vec![].into());

    assert_eq!(t.as_data_ref().unwrap(), [] as [u8; 0]);
}

#[test]
fn as_data_mut_begin_object() {
    let mut t = Token::BeginObject(4711);

    assert!(t.as_data_mut().is_none());
}

#[test]
fn as_data_mut_end_object() {
    let mut t = Token::EndObject;

    assert!(t.as_data_mut().is_none());
}

#[test]
fn as_data_mut_begin_array() {
    let mut t = Token::BeginArray(4711);

    assert!(t.as_data_mut().is_none());
}

#[test]
fn as_data_mut_end_array() {
    let mut t = Token::EndArray;

    assert!(t.as_data_mut().is_none());
}

#[test]
fn as_data_mut_data() {
    let mut t = Token::Data(vec![].into());

    assert_eq!(t.as_data_mut().unwrap(), &vec![].into());
}

#[test]
fn as_data_mut_err_begin_object() {
    let mut t = Token::BeginObject(4711);
    let err = t.as_data_mut_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "BeginObject"));
}

#[test]
fn as_data_mut_err_end_object() {
    let mut t = Token::EndObject;
    let err = t.as_data_mut_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "EndObject"));
}

#[test]
fn as_data_mut_err_begin_array() {
    let mut t = Token::BeginArray(4711);
    let err = t.as_data_mut_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "BeginArray"));
}

#[test]
fn as_data_mut_err_end_array() {
    let mut t = Token::EndArray;
    let err = t.as_data_mut_err().unwrap_err();

    assert!(matches!(err, Error::UnexpectedEvent { expected, found }
        if expected == "Data" && found == "EndArray"));
}

#[test]
fn as_data_mut_err_data() {
    let mut t = Token::Data(vec![].into());

    assert_eq!(t.as_data_mut_err().unwrap(), &vec![].into());
}
