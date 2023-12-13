use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// attribute-type proc macro that can be added to a tests mod to declare a 
// derive-type proc macro that derives the tested version of a trait using those tests
#[proc_macro_attribute]
pub fn derive_tested_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("args: {args}");
    println!("input: {input}");

    input
}

//#[derive_tested_trait(Container)]
//mod container_tests {
//    use super::UntestedContainer;
//
//    #[test]
//    fn test_returns_correct_num_items() {
//        #[arbitrary(default)]
//        let c: Container<usize> = Container::new(Box::new([0,1,2,3]));
//
//        assert_eq!(c.len(), c.into_iter().count());
//
//    }
//}

// This is what should be produced from the above
#[proc_macro_derive(Container)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Container #impl_generics for #type_name #ty_generics #where_clause{}

        #[cfg(test)]
        mod container_tests {
            use super::UntestedContainer;
            use super::#type_name; // TODO the outer macro needs to insert this into the ItemMod's content vec
            
            #[test]
            fn test_returns_correct_num_items() {
                // TODO how to resolve the type here? Maybe every instance of the trait name (e.g. Container) is replaced with the concrete type?
                let c: #type_name<usize> = #type_name::new(Box::new([0,1,2,3]));

                assert_eq!(c.len(), c.into_iter().count());
            }
        }
    };

    TokenStream::from(expanded)
}
