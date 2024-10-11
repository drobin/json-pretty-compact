# json-pretty-compact: A compact & pretty formatter for [serde_json].

## Introduction

The `json_pretty_compact` crate implements a pretty [serde_json] formatter
which tries to be as compact as possible. This can increase the readability
of formatted JSON.

A compact-pretty formatted JSON looks like this:

```json
[
  {
    "name": { "first": "Kobe", "middle": "Nico", "last": "Grimes" },
    "phoneNumber": "1-547-479-5471 x062",
    "username": "Kobe-Grimes",
    "emails": [ "Melyssa.Cremin4@gmail.com", "Jayne.Green37@gmail.com" ],
    "coordinates": { "latitude": "-66.3821", "longitude": "127.117" }
  },
  {
    "name": { "first": "Adrian", "middle": "Finley", "last": "Koch" },
    "phoneNumber": "1-420-853-5251 x68083",
    "username": "Adrian-Koch",
    "emails": [ "Andy99@gmail.com", "Elenor.Aufderhar96@gmail.com" ],
    "coordinates": { "latitude": "51.4003", "longitude": "3.351" }
  }
]
```

## Compaction rules

The formatter tries to put arrays and objects into one line, as long as the
line length is still within its limit. If a line will become too large, the
formatter will change into a pretty format.

## Usage

```rust
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
```

## License

> You can check out the full license
> [here](https://github.com/drobin/json-pretty-compact/blob/main/LICENSE).

This project is licensed under the terms of the **MIT** license.

[serde_json]: https://docs.rs/serde_json/latest/serde_json/index.html
