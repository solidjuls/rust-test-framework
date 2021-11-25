extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn test_custom(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let block = &input_fn.block;
    let name = input_fn.sig.ident;

    let code = quote! {
        register_test! {
            testframework::Test {
                name: std::stringify!(#name),
                file: std::file!(),
                line: std::line!(),
                handler: std::boxed::Box::new(|| {
                    #block
                }),
            }
        }
    };

    TokenStream::from(code)
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
