//// Defining trait
use trait_test_macro::derive_tested_trait;

pub trait UntestedContainer <T>: IntoIterator{
    fn new (items: Box<[T]>) -> Self;
    fn len (&self) -> usize;
}

#[derive_tested_trait]
trait Container <T>: UntestedContainer<T>{
    //use super::UntestedContainer;
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
