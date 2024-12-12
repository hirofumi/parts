use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::token::Pub;
use syn::{parse_macro_input, DeriveInput, Error, Visibility};

#[proc_macro_derive(Parts)]
pub fn derive(input: TokenStream) -> TokenStream {
    const DERIVATION: &str = "Parts";

    let mut input = parse_macro_input!(input as DeriveInput);
    let span = input.span();

    let compile_error = |message| TokenStream::from(Error::new(span, message).to_compile_error());
    let not_supported = |types| compile_error(format!("{DERIVATION} is not supported for {types}"));

    let original_var = quote! { original };
    let original_ty = input.ident;
    let parts_ty = format_ident!("{}{DERIVATION}", original_ty);
    let parts_from_original;

    input.ident = parts_ty.clone();
    match &mut input.data {
        syn::Data::Struct(ref mut data) => match &mut data.fields {
            syn::Fields::Named(ref mut fields) => {
                let mut initializers = vec![];
                for field in &mut fields.named {
                    let span = field.vis.span();
                    let field_name = &field.ident;
                    field.vis = Visibility::Public(Pub { span });
                    initializers.push(quote! { #field_name: #original_var.#field_name, });
                }
                parts_from_original = quote! {
                    Self { #(#initializers)* }
                };
            }
            syn::Fields::Unnamed(ref mut fields) => {
                let mut initializers = vec![];
                let mut i = 0;
                for field in &mut fields.unnamed {
                    let span = field.vis.span();
                    field.vis = Visibility::Public(Pub { span });
                    initializers.push(quote! { #original_var.#i, });
                    i += 1;
                }
                parts_from_original = quote! {
                    Self(#(#initializers)*)
                };
            }
            syn::Fields::Unit => return not_supported("unit structs"),
        },
        syn::Data::Enum(_) => return not_supported("enums"),
        syn::Data::Union(_) => return not_supported("unions"),
    };

    TokenStream::from(quote! {
        #input

        impl #original_ty {
            pub fn into_parts(self) -> #parts_ty {
                self.into()
            }
        }

        impl ::std::convert::From<#original_ty> for #parts_ty {
            fn from(#original_var: #original_ty) -> Self {
                #parts_from_original
            }
        }
    })
}
