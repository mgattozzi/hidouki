extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::Item;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut stream = proc_macro2::TokenStream::from(attr).into_iter();
    let method = {
        match stream.next().expect("No arguments passed to route") {
            TokenTree::Ident(ident) => ident,
            _ => panic!("No valld http verb ident in route macro"),
        }
    };
    let path = {
        match stream.next().expect("Not enough arguments passed to route") {
            TokenTree::Literal(lit) => lit,
            _ => panic!("No valld http path in route macro"),
        }
    };
    if let Item::Fn(item) = syn::parse(input).expect("Unable to parse item") {
        let name = item.sig.ident;
        let block = item.block;
        let tokens = quote! {
            #[allow(non_camel_case_types)]
            struct #name;

            impl hidouki::router::Route for #name {
                const PATH: &'static str = #path;
                const METHOD: hidouki::Method = hidouki::Method::#method;
                fn route(req: hidouki::Request<String>) -> hidouki::router::JoinHandle<hidouki::Result<hidouki::Response<String>>> {
                    use hidouki::{Response,Request};
                    hidouki::spawn(async #block)
                }
            }
        };
        tokens.into()
    } else {
        panic!("Route attribute was not use on an async function");
    }
}
