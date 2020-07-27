use proc_macro2::TokenStream;
use syn;

use crate::models::{HasStrumVariantProperties, HasTypeProperties};

pub fn to_string_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("ToString only works on Enums"),
    };

    let type_meta = ast.get_type_properties();
    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        let meta = variant.get_variant_properties();

        if meta.is_disabled {
            continue;
        }

        // Look at all the serialize attributes.
        let output = meta.get_preferred_name(type_meta.case_style);

        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        arms.push(quote! { #name::#ident #params => ::std::string::String::from(#output) });
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => panic!("to_string() called on disabled variant.")})
    }

    quote! {
        impl #impl_generics ::std::string::ToString for #name #ty_generics #where_clause {
            fn to_string(&self) -> ::std::string::String {
                match *self {
                    #(#arms),*
                }
            }
        }
    }
}