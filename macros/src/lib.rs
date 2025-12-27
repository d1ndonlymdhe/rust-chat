use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemEnum, ItemFn, Variant, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn db_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Create the new argument
    let pool_arg: FnArg = parse_quote! { pool: &sqlx::SqlitePool };

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
    let x: Variant = parse_quote! { Sqlx(::sqlx::Error)};
    // input.variants.push(x);
    input.variants.push(x);
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
