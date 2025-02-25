/*!
Implementation details for `bitflags-derive`.

This library isn't intended to be used directly.
*/

#![deny(missing_docs)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

mod debug;
mod display;
mod from_str;

#[cfg(feature = "serde")]
mod serde;

/**
Derive [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html).

This macro will use [`to_writer`](https://docs.rs/bitflags/latest/bitflags/parser/fn.to_writer.html) to
format flags values.
*/
#[proc_macro_derive(FlagsDebug)]
pub fn derive_bitflags_debug(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    debug::expand(syn::parse_macro_input!(item as syn::DeriveInput)).unwrap_or_compile_error()
}

/**
Derive [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html).

This macro will use [`to_writer`](https://docs.rs/bitflags/latest/bitflags/parser/fn.to_writer.html) to
format flags values.
*/
#[proc_macro_derive(FlagsDisplay)]
pub fn derive_bitflags_display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    display::expand(syn::parse_macro_input!(item as syn::DeriveInput)).unwrap_or_compile_error()
}

/**
Derive [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

This macro will use [`from_str`](https://docs.rs/bitflags/latest/bitflags/parser/fn.from_str.html) to
parse flags values.
*/
#[proc_macro_derive(FlagsFromStr)]
pub fn derive_bitflags_from_str(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_str::expand(syn::parse_macro_input!(item as syn::DeriveInput)).unwrap_or_compile_error()
}

/**
Derive [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html).
*/
#[cfg(feature = "serde")]
#[proc_macro_derive(FlagsSerialize)]
pub fn derive_bitflags_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    serde::serialize::expand(syn::parse_macro_input!(item as syn::DeriveInput))
        .unwrap_or_compile_error()
}

/**
Derive [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html).
*/
#[cfg(feature = "serde")]
#[proc_macro_derive(FlagsDeserialize)]
pub fn derive_bitflags_deserialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    serde::deserialize::expand(syn::parse_macro_input!(item as syn::DeriveInput))
        .unwrap_or_compile_error()
}

trait ResultExt {
    fn unwrap_or_compile_error(self) -> proc_macro::TokenStream;
}

impl ResultExt for Result<proc_macro2::TokenStream, syn::Error> {
    fn unwrap_or_compile_error(self) -> proc_macro::TokenStream {
        proc_macro::TokenStream::from(match self {
            Ok(item) => item,
            Err(err) => err.into_compile_error(),
        })
    }
}
