/*!
Implementation details for `bitflags-derive`.

This library isn't intended to be used directly.
*/

#![deny(missing_docs)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

mod debug;

/**
Derive [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) for a flags type.

This macro will use [`https://docs.rs/bitflags/latest/bitflags/parser/fn.to_writer.html`] to
format flags values.
*/
#[proc_macro_derive(FlagsDebug)]
pub fn derive_bitflags_debug(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    debug::expand(syn::parse_macro_input!(item as syn::DeriveInput)).unwrap_or_compile_error()
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
