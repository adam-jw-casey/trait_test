use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

// attribute-type proc macro that can be added to a tests mod to declare a 
// derive-type proc macro that derives the tested version of a trait using those tests
#[proc_macro_attribute]
pub fn derive_tested_trait(args: TokenStream, trait_input: TokenStream) -> TokenStream {
    println!("args: {args}");
    println!("trait_input: {trait_input}");

    trait_input
}

//#[derive_tested_trait]
// trait Container <T>: UntestedContainer<T>{
//    fn do_not_manually_implement();
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
pub fn derive(derive_input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(derive_input as DeriveInput);

    let type_name = derive_input.ident.clone();

    let mod_name = Ident::new(&format!("{}_test", "Container"), Span::call_site());

    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Container #impl_generics for #type_name #ty_generics #where_clause{
            fn do_not_manually_implement(){}
        }

        #[cfg(test)]
        mod #mod_name {
            use super::UntestedContainer; // TODO This is a trait bound on the trait being tested from the outer macro
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
