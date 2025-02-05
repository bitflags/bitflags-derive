use proc_macro2::TokenStream;

pub(crate) fn expand(item: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    Ok(
        quote!(impl #impl_generics bitflags_derive::__private::core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut bitflags_derive::__private::core::fmt::Formatter) -> bitflags_derive::__private::core::fmt::Result {
                bitflags_derive::__private::bitflags::parser::to_writer(self, f)
            }
        }),
    )
}
