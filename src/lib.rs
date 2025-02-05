/*!
Custom derive attributes for types generated by [`bitflags`](https://docs.rs/bitflags).

## Usage

Add `bitflags-derive` to your `Cargo.toml` alongside `bitflags`:

```toml
[dependencies.bitflags-derive]
version = "0.0.2"

[dependencies.bitflags]
version = "2"
```

Inside the [`bitflags!` macro](https://docs.rs/bitflags/latest/bitflags/macro.bitflags.html), add a `#[derive]` attribute with any traits from `bitflags-derive` that you'd like to support:

```rust
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitflags_derive;

# fn main() -> Result<(), bitflags::parser::ParseError> {
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
# Ok(())
# }
```

These derives work for any type that implements the [`Flags`](https://docs.rs/bitflags/latest/bitflags/trait.Flags.html) trait.
*/

#![no_std]
#![deny(missing_docs)]

#[doc(inline)]
pub use bitflags_derive_macros::*;

#[doc(hidden)]
pub mod __private {
    pub use bitflags;
    pub use core;
}
