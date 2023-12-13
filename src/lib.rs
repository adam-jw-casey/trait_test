use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

// attribute-type proc macro that can be added to a tests mod to declare a 
// derive-type proc macro that derives the tested version of a trait using those tests
#[proc_macro_attribute]
pub fn derive_tested_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("args: {}", args);
    println!("input: {}", input);

    input
}

// This is what should be produced from the above
#[proc_macro_derive(Container)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident.clone();
    let test_mod_name = Ident::new(&format!("{}_test", type_name), Span::call_site());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        // #[derive(Arbitrary)] TODO

        impl #impl_generics Container #impl_generics for #type_name #ty_generics #where_clause{}

        #[cfg(test)]
        mod #test_mod_name {
            use super::UntestedContainer;
            use super::#type_name;
            //use arbitrary::Arbitrary; TODO
            
            #[test]
            fn test_returns_correct_num_items() {
                // #[arbitrary(default)] TODO
                let c: #type_name<usize> = #type_name::new(Box::new([0,1,2]));

                assert_eq!(c.len(), c.into_iter().count());
            }
        }
    };

    TokenStream::from(expanded)
}
