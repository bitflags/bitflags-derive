# `bitflags-derive`

[![ci](https://github.com/bitflags/bitflags-derive/actions/workflows/ci.yml/badge.svg)](https://github.com/bitflags/bitflags-derive/actions/workflows/ci.yml)

This project is a set of custom derive attributes for [`bitflags`](https://docs.rs/bitflags) that generates flags-aware implementations.

## Usage

Add `bitflags-derive` to your `Cargo.toml` alongside `bitflags`:

```toml
[dependencies.bitflags-derive]
version = "0.0.4"

[dependencies.bitflags]
version = "2"
```

Inside the [`bitflags!` macro](https://docs.rs/bitflags/latest/bitflags/macro.bitflags.html), add a `#[derive]` attribute with any traits from `bitflags-derive` that you'd like to support:

```rust
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitflags_derive;

bitflags! {
    #[derive(FlagsDisplay, FlagsFromStr)]
    struct MyFlags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}

let flags = "A | B".parse::<MyFlags>()?;

assert_eq!("A | B", flags.to_string());
```

See [the docs](https://docs.rs/bitflags-derive) for details on all supported attributes.
