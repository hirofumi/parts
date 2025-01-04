use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::token::Pub;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Meta, Visibility};

#[proc_macro_derive(Parts, attributes(parts_attr))]
pub fn derive(input: TokenStream) -> TokenStream {
    expand(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.into_compile_error().into())
}

fn expand(mut input: DeriveInput) -> syn::Result<TokenStream> {
    const DERIVATION: &str = "Parts";

    let span = input.span();
    let unsupported = |types| {
        Err(syn::Error::new(
            span,
            format!("{DERIVATION} is not supported for {types}"),
        ))
    };

    let original_var = quote! { original };
    let original_ty = input.ident;
    let parts_ty = format_ident!("{}{DERIVATION}", original_ty);
    let parts_from_original;

    input.ident = parts_ty.clone();

    input.attrs = input
        .attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident("parts_attr"))
        .map(|attr| {
            attr.parse_args_with(Meta::parse)
                .map(|meta| Attribute { meta, ..attr })
        })
        .collect::<syn::Result<_>>()?;

    match &mut input.data {
        Data::Struct(ref mut data) => match &mut data.fields {
            Fields::Named(ref mut fields) => {
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
            Fields::Unnamed(ref mut fields) => {
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
            Fields::Unit => return unsupported("unit structs"),
        },
        Data::Enum(_) => return unsupported("enums"),
        Data::Union(_) => return unsupported("unions"),
    };

    Ok(TokenStream::from(quote! {
        #input

        #[automatically_derived]
        impl #original_ty {
            pub fn into_parts(self) -> #parts_ty {
                self.into()
            }
        }

        #[automatically_derived]
        impl ::std::convert::From<#original_ty> for #parts_ty {
            fn from(#original_var: #original_ty) -> Self {
                #parts_from_original
            }
        }
    }))
}
