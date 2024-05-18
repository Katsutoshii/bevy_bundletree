extern crate proc_macro;

mod bundle_enum {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Data, DataEnum, DeriveInput, Error, Result};

    pub(crate) fn derive(input: &DeriveInput) -> Result<TokenStream> {
        match &input.data {
            Data::Enum(data) => impl_enum(input, data),
            _ => Err(Error::new_spanned(
                input,
                "From can only be derived for enums",
            )),
        }
    }

    fn impl_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
        let ty = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let from_impls = data
            .variants
            .iter()
            .filter(|variant| variant.fields.len() == 1)
            .map(|variant| {
                let variant_ident = &variant.ident;
                let field = &variant.fields.iter().next().unwrap();
                let field_ty = &field.ty;

                quote! {
                    impl #impl_generics From<#field_ty> for #ty #ty_generics #where_clause {
                        fn from(value: #field_ty) -> Self {
                            #ty::#variant_ident(value)
                        }
                    }
                }
            });
        let spawn_mappings = data
            .variants
            .iter()
            .filter(|variant| variant.fields.len() == 1)
            .map(|variant| {
                let variant_ident = &variant.ident;
                quote! {
                    Self::#variant_ident(bundle) => commands.spawn(bundle),
                }
            });
        Ok(quote! {
            impl BundleEnum for #ty {
                fn spawn<'c>(self, commands: &'c mut Commands) -> bevy::ecs::system::EntityCommands<'c> {
                    match self {
                        #(#spawn_mappings)*
                    }
                }
            }
            #(#from_impls)*
        })
    }
}

mod as_bundle_tree {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Data, DataEnum, DeriveInput, Error, Result};

    pub(crate) fn derive(input: &DeriveInput) -> Result<TokenStream> {
        match &input.data {
            Data::Enum(data) => impl_enum(input, data),
            _ => Err(Error::new_spanned(
                input,
                "From can only be derived for enums",
            )),
        }
    }

    fn impl_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
        let ty = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let as_bundle_tree_impls = data
            .variants
            .iter()
            .filter(|variant| variant.fields.len() == 1)
            .map(|variant| {
                let field = &variant.fields.iter().next().unwrap();
                let field_ty = &field.ty;

                quote! {
                    impl #impl_generics IntoBundleTree<#ty #ty_generics> for #field_ty #where_clause {}
                }
            });
        Ok(quote! {
            #(#as_bundle_tree_impls)*
        })
    }
}

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BundleEnum)]
pub fn derive_bundle_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    bundle_enum::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(IntoBundleTree)]
pub fn derive_as_bundle_tree(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    as_bundle_tree::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
