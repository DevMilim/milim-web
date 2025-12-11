extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn milim_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parseia a função
    let input = parse_macro_input!(item as ItemFn);

    // garante que o usuário só use em async fn
    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            input.sig.ident,
            "#[milim_handler] só pode ser usado em `async fn`",
        )
        .to_compile_error()
        .into();
    }

    // mantém atributos/visibilidade/ident/inputs/block
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    // cria uma nova assinatura sem `async` e com retorno:
    // -> Pin<Box<dyn Future<Output = ()> + Send + '_>>
    let mut new_sig = sig.clone();
    new_sig.asyncness = None;
    new_sig.output = syn::parse_quote! {
        -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send + '_>>
    };

    // gera código: a função passa a retornar Box::pin(async move { original body })
    let expanded = quote! {
        #(#attrs)*
        #vis #new_sig {
            Box::pin(async move #block)
        }
    };

    expanded.into()
}
