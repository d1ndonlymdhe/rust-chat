use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, FnArg, ItemEnum, ItemFn, Variant, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn db_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Create the new argument
    let pool_arg: FnArg = parse_quote! { pool: &sqlx::PgPool };

    // Insert it at the beginning of the inputs
    input.sig.inputs.insert(0, pool_arg);

    // Generate the output
    let output = quote! {
        #input
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_err(_attr:TokenStream, item: TokenStream) -> TokenStream{
    let mut input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;
    let sqlx_error_variant: Variant = parse_quote! { Sqlx(::sqlx::Error)};
    let any_cast_attr: Attribute = parse_quote! {
        #[::macros::any_cast]
    };
    // input.variants.push(x);
    input.variants.push(sqlx_error_variant);
    input.attrs.push(any_cast_attr);
    let output = quote! {
        #input
        impl From<sqlx::Error> for #enum_name {
            fn from(err: sqlx::Error) -> Self {
                #enum_name::Sqlx(err)
            }
        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn any_cast(_attr:TokenStream,item:TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;
    let output = quote! {
        #input
        impl From<#enum_name> for AnyErr {
            fn from(value: #enum_name) -> AnyErr {
                return AnyErr(());
            }
        }   
    };
    output.into()
}
