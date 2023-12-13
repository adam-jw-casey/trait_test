use trait_test::Container;

trait UntestedContainer <T>: IntoIterator{
    fn new (items: Box<[T]>) -> Self;
    fn len (&self) -> usize;
}

trait Container <T>: UntestedContainer<T>{}

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
