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
use crate::token::Token;

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
    let t = Token::Data(vec![]);

    assert!(t.as_begin_object().is_none());
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
    let t = Token::Data(vec![]);

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
    let t = Token::Data(vec![]);

    assert!(t.as_begin_array().is_none());
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
    let t = Token::Data(vec![]);

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
    let t = Token::Data(vec![]);

    assert_eq!(t.as_data().unwrap(), [] as [u8; 0]);
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
    let t = Token::Data(vec![]);

    assert_eq!(t.as_data_err().unwrap(), [] as [u8; 0]);
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
    let mut t = Token::Data(vec![]);

    assert_eq!(t.as_data_mut().unwrap(), &mut Vec::<u8>::new());
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
    let mut t = Token::Data(vec![]);

    assert_eq!(t.as_data_mut_err().unwrap(), &mut Vec::<u8>::new());
}
