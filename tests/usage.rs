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

#[test]
fn basic_usage() {
    use json_pretty_compact::PrettyCompactFormatter;
    use serde::Serialize;
    use serde_json::{Serializer, Value};

    // Create a JSON value.
    // In this simple example it contains only the "true" value.
    let value: Value = serde_json::from_str("true").unwrap();

    // The buffer where the serialized JSON is written.
    let mut target = vec![];

    // Create the formatter.
    // It takes all the default values.
    let formatter = PrettyCompactFormatter::new();

    // Serialize the JSON value into the `target` buffer.
    let mut ser = Serializer::with_formatter(&mut target, formatter);
    value.serialize(&mut ser).unwrap();

    assert_eq!(target, b"true");
}
