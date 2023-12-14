//// Defining trait
use trait_test::derive_tested_trait;

trait UntestedContainer <T>: IntoIterator{
    fn new (items: Box<[T]>) -> Self;
    fn len (&self) -> usize;
}
#[derive_tested_trait]
trait Container <T>: UntestedContainer<T>{
    use super::UntestedContainer;
    #[test]
    fn test_returns_correct_num_items() {
        let c: Container<usize> = Container::new(Box::new([0,1,2,3]));

        assert_eq!(c.len(), c.into_iter().count());
    }

    #[test]
    fn test_deterministic_constructor() {
        let c1: Container<usize> = Container::new(Box::new([0,1,2,3]));
        let c2: Container<usize> = Container::new(Box::new([0,1,2,3]));

        assert_eq!(c1.len(), c2.len());
    }
}

//// This is what should be produced from the above
//#[proc_macro_derive(Container)]
//pub fn derive(derive_input: TokenStream) -> TokenStream {
//    let derive_input = parse_macro_input!(derive_input as DeriveInput);
//
//    let type_name = derive_input.ident.clone();
//
//    let mod_name = Ident::new(&format!("{}_test", "Container"), Span::call_site());
//
//    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
//
//    let expanded = quote! {
//        impl #impl_generics Container #impl_generics for #type_name #ty_generics #where_clause{
//            fn do_not_manually_implement(){}
//        }
//
//        #[cfg(test)]
//        mod #mod_name {
//            use super::UntestedContainer; // TODO This is a trait bound on the trait being tested from the outer macro
//            use super::#type_name; // TODO the outer macro needs to insert this into the ItemMod's content vec
//            
//            #[test]
//            fn test_returns_correct_num_items() {
//                // TODO how to resolve the type here? Maybe every instance of the trait name (e.g. Container) is replaced with the concrete type?
//                let c: #type_name<usize> = #type_name::new(Box::new([0,1,2,3]));
//
//                assert_eq!(c.len(), c.into_iter().count());
//            }
//        }
//    };
//
//    TokenStream::from(expanded)
//}

//// End user (implementing trait) below

impl <T> UntestedContainer<T> for Pile<T>{
    fn new (items: Box<[T]>) -> Self {
        Self{contents: Vec::from(items)}
    }

    fn len (&self) -> usize {
        self.contents.len()
    }
}

impl <T> IntoIterator for Pile<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

#[derive(Container)]
struct Pile<T>{
    contents: Vec<T>,
}
