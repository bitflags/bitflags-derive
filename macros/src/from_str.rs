use proc_macro2::TokenStream;

pub(crate) fn expand(item: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    Ok(
        quote!(impl #impl_generics bitflags_derive::__private::core::str::FromStr for #ident #ty_generics #where_clause {
            type Err = bitflags_derive::__private::bitflags::parser::ParseError;

            fn from_str(v: &str) -> bitflags_derive::__private::core::result::Result<Self, bitflags_derive::__private::bitflags::parser::ParseError> {
                bitflags_derive::__private::bitflags::parser::from_str(v)
            }
        }),
    )
}
