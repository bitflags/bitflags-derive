pub(crate) mod serialize {
    use proc_macro2::TokenStream;

    pub(crate) fn expand(item: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
        let ident = item.ident;
        let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

        Ok(
            quote!(impl #impl_generics bitflags_derive::__private::serde::Serialize for #ident #ty_generics #where_clause {
                fn serialize<S: bitflags_derive::__private::serde::Serializer>(&self, serializer: S) -> bitflags_derive::__private::core::result::Result<S::Ok, S::Error> {
                    bitflags_derive::__private::serde::serialize(self, serializer)
                }
            }),
        )
    }
}

pub(crate) mod deserialize {
    use proc_macro2::{Span, TokenStream};

    pub(crate) fn expand(item: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
        let ident = item.ident;

        let de_lt = syn::Lifetime::new("'bitflags_derive_de", Span::call_site());

        let mut de_generics = item.generics.clone();
        de_generics
            .params
            .push_value(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                de_lt.clone(),
            )));

        let (impl_generics, _, where_clause) = de_generics.split_for_impl();
        let (_, ty_generics, _) = item.generics.split_for_impl();

        Ok(
            quote!(impl #impl_generics bitflags_derive::__private::serde::Deserialize<#de_lt> for #ident #ty_generics #where_clause {
                fn deserialize<D: bitflags_derive::__private::serde::Deserializer<#de_lt>>(deserializer: D) -> bitflags_derive::__private::core::result::Result<Self, D::Error> {
                    bitflags_derive::__private::serde::deserialize(deserializer)
                }
            }),
        )
    }
}
