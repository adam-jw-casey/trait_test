use trait_test::Container;

trait UntestedContainer <T>: IntoIterator{
    fn new (items: Box<[T]>) -> Self;
    fn len (&self) -> usize;
}

trait Container <T>: UntestedContainer<T>{}

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
