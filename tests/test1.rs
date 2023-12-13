use trait_test::{
    Container,
    derive_tested_trait,
};

trait UntestedContainer <T>: IntoIterator{
    fn new (items: Box<[T]>) -> Self;
    fn len (&self) -> usize;
}

#[derive_tested_trait]
trait Container <T>: UntestedContainer<T>{
      fn do_not_manually_implement();
//
//    #[test]
//    fn test_returns_correct_num_items() {
//        let c: Container<usize> = Container::new(Box::new([0,1,2,3]));
//
//        assert_eq!(c.len(), c.into_iter().count());
//    }
//
//    #[test]
//    fn test_deterministic_constructor() {
//        let c1: Container<usize> = Container::new(Box::new([0,1,2,3]));
//        let c2: Container<usize> = Container::new(Box::new([0,1,2,3]));
//
//        assert_eq!(c1.len(), c.len());
//    }
}

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
