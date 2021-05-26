extern crate proc_macro;

use proc_macro::TokenStream;
use quote::queote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DerviceInput) -> TokenStream {
    let name = &ast.indent;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! my name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
