use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro]
pub fn mx(input: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    let pushes = exprs.iter().map(|expr| {
        quote! {
            __v.push(std::sync::Arc::new(#expr) as std::sync::Arc<dyn milim_web::router::Middleware>);
        }
    });
    let expanded = quote! {{
        let mut __v: Vec<std::sync::Arc<dyn milim_web::router::Middleware>> = Vec::new();
        #(#pushes)*
        __v
    }};
    expanded.into()
}
