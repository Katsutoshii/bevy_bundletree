extern crate proc_macro;

mod expand {
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
                let field_has_from = field.attrs.iter().any(|attr| attr.path.is_ident("from"));

                if field_has_from {
                    Some(quote! {
                        impl #impl_generics From<#field_ty> for #ty #ty_generics #where_clause {
                            fn from(value: #field_ty) -> Self {
                                #ty::#variant_ident(value)
                            }
                        }
                    })
                } else {
                    None
                }
            });

        Ok(quote! {
            #(#from_impls)*
        })
    }
}

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(From, attributes(from))]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
