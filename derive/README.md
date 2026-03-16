# oxicode_derive

Procedural macros for deriving `Encode` and `Decode` traits for the [OxiCode](https://crates.io/crates/oxicode) binary serialization library.

## Overview

This crate provides derive macros that automatically implement the `Encode` and `Decode` traits for your custom types. It is part of the OxiCode ecosystem and is typically used through the main `oxicode` crate with the `derive` feature enabled.

## Usage

Add `oxicode` to your `Cargo.toml` with the `derive` feature:

```toml
[dependencies]
oxicode = { version = "0.2", features = ["derive"] }
```

Then use the derive macros on your types:

```rust
use oxicode::{Encode, Decode};

#[derive(Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

## Supported Types

The derive macros support:

- **Structs** with named fields, unnamed fields (tuple structs), or unit structs
- **Enums** with any combination of named, unnamed, and unit variants
- **Generics** with full lifetime and type parameter support
- **Where clauses** and bounds

## Example

```rust
use oxicode::{encode, decode, Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct User<'a> {
    id: u64,
    name: &'a str,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Pending { reason: String },
}

fn main() -> Result<(), oxicode::Error> {
    let user = User {
        id: 42,
        name: "Alice",
        active: true,
    };

    let bytes = encode(&user)?;
    let decoded: User = decode(&bytes)?;

    assert_eq!(user, decoded);
    Ok(())
}
```

## Features

- **Zero-cost abstractions**: Generated code is as efficient as hand-written implementations
- **Generic support**: Full support for generic types with automatic trait bounds
- **Lifetime support**: Works seamlessly with borrowed data
- **Error handling**: Proper error propagation with `Result` types

## Limitations

- **Unions**: Derive macros cannot be used on unions due to safety concerns
- **Manual implementations**: For complex scenarios, you may need to implement `Encode` and `Decode` manually

## Documentation

For detailed documentation, see the [OxiCode crate documentation](https://docs.rs/oxicode).

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](../LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/cool-japan/oxicode) for contribution guidelines.
