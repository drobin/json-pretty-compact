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

use json_pretty_compact::PrettyCompactFormatter;
use serde::Serialize;
use serde_json::{Serializer, Value};
use std::cmp;
use std::fs::{self, File};
use std::io::Cursor;
use std::path::PathBuf;

pub fn fixture_path(name: &str, extension: &str) -> PathBuf {
    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    let mut path: PathBuf = [MANIFEST_DIR, "fixtures", name].iter().collect();

    assert!(path.set_extension(extension));

    path
}

pub fn parse_json(name: &str) -> Value {
    let path = fixture_path(name, "json");
    let file = File::open(path).unwrap();

    serde_json::from_reader(file).unwrap()
}

pub fn parse_json_string(json: &str) -> Value {
    serde_json::from_str(json).unwrap()
}

pub fn serialize_to_string(value: &Value, formatter: PrettyCompactFormatter) -> String {
    let mut cursor = Cursor::new(vec![]);
    let mut ser = Serializer::with_formatter(&mut cursor, formatter);

    value.serialize(&mut ser).unwrap();

    String::from_utf8(cursor.into_inner()).unwrap()
}

pub fn fixture_to_string(name: &str, extension: &str) -> String {
    let path = fixture_path(name, extension);

    fs::read_to_string(path).unwrap()
}

pub fn print_table(left: &str, right: &str, all: bool) {
    let lwidth = cmp::max(left.lines().count(), right.lines().count())
        .to_string()
        .len();

    let width = left
        .lines()
        .chain(right.lines())
        .map(|s| s.len())
        .max()
        .unwrap();

    let mut left_lines = left.lines();
    let mut right_lines = right.lines();
    let zip = (&mut left_lines).zip(&mut right_lines);

    for (idx, (l, r)) in zip.enumerate() {
        let eq = if l == r { "EQ" } else { "NE" };

        if all || eq != "EQ" {
            println!(
                "| {:>lwidth$} | {:<width$} | {:<width$} | {} |",
                idx + 1,
                l,
                r,
                eq
            );
        }
    }

    for s in left_lines {
        println!("| {:>lwidth$} | {:<width$} | {:<width$} | NE |", "", s, "");
    }

    for s in right_lines {
        println!("| {:>lwidth$} | {:<width$} | {:<width$} | NE |", "", "", s,);
    }
}

macro_rules! t {
    ($name:ident, $fixture:literal, $extension:literal, $formatter:expr) => {
        #[test]
        fn $name() {
            use $crate::common::*;

            let value = parse_json($fixture);

            let json = serialize_to_string(&value, $formatter);
            let expected = fixture_to_string($fixture, $extension);

            if json != expected {
                print_table(&json, &expected, false);
            }

            assert_eq!(json, expected);

            let value_out = parse_json_string(&json);

            assert_eq!(value, value_out);
        }
    };
}

pub(crate) use t;
