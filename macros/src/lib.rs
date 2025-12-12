use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let orig_name = &sig.ident;
    let inner_name = syn::Ident::new(&format!("__{}_inner", orig_name), orig_name.span());
    let inputs = &sig.inputs;
    let output = &sig.output;
    let block = &input_fn.block;

    let mut args_types = vec![];

    for arg in inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Type::Reference(ty_ref) = &*pat_type.ty {
                let inner_ty = &*ty_ref.elem;
                args_types.push(quote! {#inner_ty});
            } else {
                let t = &*pat_type.ty;
                args_types.push(quote! {#t});
            }
        }
    }

    let req_ty = &args_types[0];
    let ctx_ty = &args_types[1];

    let expanded = quote! {
        #vis async fn #inner_name(#inputs) #output{
            #block
        }

        type __HandlerBoxFut = std::pin::Pin<Box<dyn std::future::Future<Output = Box<dyn milim_web::responder::Responder + Send>> + Send + 'static>>;

        #vis fn #orig_name(req: std::sync::Arc<#req_ty>, ctx: std::sync::Arc<#ctx_ty>) -> __HandlerBoxFut{
            Box::pin(async move{
                let r = #inner_name(&*req, &*ctx).await;
                Box::new(r) as Box<dyn milim_web::responder::Responder+Send>
            })
        }
    };
    TokenStream::from(expanded)
}
