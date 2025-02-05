# `bitflags-derive`

[![ci](https://github.com/bitflags/bitflags-derive/actions/workflows/ci.yml/badge.svg)](https://github.com/bitflags/bitflags-derive/actions/workflows/ci.yml)

This project is a set of custom derive attributes for [`bitflags`](https://docs.rs/bitflags) that generates flags-aware implementations.

## Usage

Add `bitflags-derive` to your `Cargo.toml` alongside `bitflags`:

```toml
[dependencies.bitflags-derive]
version = "0.0.1"

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
    #[derive(FlagsDebug)]
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}

// The regular `#[derive(Debug)]` would produce "Flags(A | B)" here
assert_eq!("A | B", format!("{:?}", Flags::A | Flags::B));
```

See [the docs](https://docs.rs/bitflags-derive) for details on all supported attributes.
