use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemFn, Attribute, Meta};

fn is_test_attribute(attr: &Attribute) -> bool {
    match &attr.meta{
        Meta::Path(path) => {
            path.segments.len() == 1 && path.segments.first().expect("We already checked that len is 1").ident == format_ident!("test")
        },
        _ => false,
    }
}

fn is_test_function(item: &TraitItem) -> bool {
    match item {
        TraitItem::Fn(item) => item.attrs.iter().filter(|a| is_test_attribute(a)).count() != 0,
        _ => true,
    }
}

// attribute-type proc macro that can be added to a tests mod to declare a 
// derive-type proc macro that derives the tested version of a trait using those tests
#[proc_macro_attribute]
pub fn derive_tested_trait(_args: TokenStream, trait_input: TokenStream) -> TokenStream {

    let trait_input = parse_macro_input!(trait_input as ItemTrait);

    let mut trait_output = trait_input.clone();
    trait_output.items.retain(|item| !is_test_function(item)); // When the trait is written out, don't include the #[test] functions, since these aren't valid Rust syntax
    
    let test_funcs: Vec<TraitItemFn> = trait_input.items.into_iter().filter(is_test_function).map(|item| match item {TraitItem::Fn(item) => item, _=> panic!("unreachable")}).collect();// TODO unused

    let trait_name = trait_input.ident;

    // These variables are for the inner macro
    let impl_generics = "impl_generics";
    let where_clause = "where_clause";
    let type_name = "type_name";
    let ty_generics = "ty_generics";
    let mod_name = "mod_name";


    let expanded = quote! {
        extern crate proc_macro;
        use proc_macro::TokenStream;
        use quote::quote;
        use syn::{parse_macro_input, DeriveInput, format_ident,};

        // This is the inner, derive macro
        #[proc_macro_derive(#trait_name)]
        pub fn derive(derive_input: TokenStream) -> TokenStream {
            let derive_input = parse_macro_input!(derive_input as DeriveInput);

            let type_name = derive_input.ident.clone();

            let mod_name = format_ident!("{}_test", #trait_name);

            let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

            let expanded = quote! {
                impl ##impl_generics ##trait_name ##impl_generics for ##type_name ##ty_generics ##where_clause{
                    fn do_not_manually_implement(){}
                }

                #[cfg(test)]
                mod #mod_name {
                    use super::UntestedContainer; // TODO This is a trait bound on the trait being tested from the outer macro
                    use super::#type_name; // TODO the outer macro needs to insert this into the ItemMod's content vec
                    
                    #[test]
                    fn test_returns_correct_num_items() {
                        // TODO how to resolve the type here? Maybe every instance of the trait name (e.g. #trait_name) is replaced with the concrete type?
                        let c: #type_name<usize> = #type_name::new(Box::new([0,1,2,3]));

                        assert_eq!(c.len(), c.into_iter().count());
                    }
                }
            };

            TokenStream::from(expanded)
        }
    };

    expanded.into()
}
