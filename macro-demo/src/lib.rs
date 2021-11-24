extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn test(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // parsing rust function to easy to use struct
    // let input_fn = parse_macro_input!(input as ItemFn);
    println!("input: {}", _metadata);
    println!("input: {}", input);
    TokenStream::from(quote! {fn dummy(){}})
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
