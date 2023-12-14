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
        TraitItem::Fn(item) => item.attrs.iter().any(is_test_attribute),
        _ => true,
    }
}

fn is_verbatim_item(item: &TraitItem) -> bool {
    matches!(item, TraitItem::Verbatim(_))
}

// attribute-type proc macro that can be added to a tests mod to declare a 
// derive-type proc macro that derives the tested version of a trait using those tests
#[allow(clippy::missing_panics_doc)]
#[proc_macro_attribute]
pub fn derive_tested_trait(_args: TokenStream, trait_input: TokenStream) -> TokenStream {

    let trait_input = parse_macro_input!(trait_input as ItemTrait);

    //// Stuff for the trait declaration
    let verbatim_items: Vec<TraitItem> = trait_input.items.clone().into_iter().filter(is_verbatim_item).collect();

    let mut trait_declaration = trait_input.clone();
    trait_declaration.items.retain(|item| !is_test_function(item) && !is_verbatim_item(item)); // When the trait is written out, don't include the #[test] functions, since these aren't valid Rust syntax
    let dummy_fn: proc_macro::TokenStream = quote!(fn do_not_manually_implement(){}).into();
    trait_declaration.items.push(parse_macro_input!(dummy_fn));

    //// Stuff for the unit tests
    let trait_name = trait_input.ident;
    let test_mod = format_ident!("{}_tests", trait_name);

    //let super_traits = trait_input.supertraits.iter().filter(||).collect();

    let test_funcs: Vec<TraitItemFn> = trait_input.items.into_iter().filter(is_test_function).map(|item| match item {TraitItem::Fn(item) => item, _=> panic!("unreachable")}).collect();// TODO unused

    // These variables are for the inner macro
    let impl_generics = "impl_generics";
    let where_clause = "where_clause";
    let type_name = "type_name";
    let ty_generics = "ty_generics";
    let unit_tests = "unit_tests";
    let derived_implementation = "derived_implementation";

    let derive_macro = quote! {
        extern crate proc_macro;
        use proc_macro::TokenStream;
        use quote::{quote, format_ident,};
        use syn::{parse_macro_input, DeriveInput};

        // This is the inner, derive macro
        #[proc_macro_derive(#trait_name)]
        pub fn derive(derive_input: TokenStream) -> TokenStream {
            let derive_input = parse_macro_input!(derive_input as DeriveInput);

            let type_name = derive_input.ident.clone();

            let mod_name = format_ident!("{}_test", "#trait_name");

            let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

            let unit_tests = quote! {
                #[cfg(test)]
                mod #test_mod {
                    //#(use super::#super_traits;) *

                    #(#verbatim_items) *

                    //use super::##type_name;

                    #(#test_funcs) *
                }
            };

            let derived_implementation = quote! {
                impl ##impl_generics #trait_name ##impl_generics for ##type_name ##ty_generics ##where_clause{
                    fn do_not_manually_implement(){}
                }
    
            };

            quote!{
                ##unit_tests
                ##derived_implementation
            }.into()
        }
    };

    quote!{
        #trait_declaration
        #derive_macro
    }.into()
}
