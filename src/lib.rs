#[cfg(test)]
mod tests {
    use crate::ExactSizedIterVec;

    #[test]
    fn it_works() {
        let content = vec![1, 2, 3, 4];
        let content2 = vec![5, 6, 7, 8];
        let content3 = vec![9, 0];
        let mut iter_vec = ExactSizedIterVec::new();
        iter_vec.push(&content);
        iter_vec.push(&content2);
        iter_vec.push(&content3);
        let flat: Vec<i32> = iter_vec.copied().collect();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], flat);
    }
}
#[derive(Debug, Default, Clone)]
pub struct ExactSizedIterVec<'a, T> {
    current_iter: usize,
    iterators: Vec<std::slice::Iter<'a, T>>,
}
impl<'a, T> ExactSizedIterVec<'a, T> {
    pub fn new() -> Self {
        Self {
            current_iter: 0,
            iterators: vec![],
        }
    }
    pub fn push(&mut self, element: &'a [T]) {
        self.iterators.push(element.iter())
    }
}
impl<'a, T> Iterator for ExactSizedIterVec<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iterators.get_mut(self.current_iter);
        if let Some(iter) = iter {
            let x = iter.next();
            if x.is_none() {
                self.current_iter += 1;
                return self
                    .iterators
                    .get_mut(self.current_iter)
                    .and_then(|x| x.next());
            } else {
                return x;
            }
        }
        None
    }
}

impl<'a, T> ExactSizeIterator for ExactSizedIterVec<'a, T> {
    fn len(&self) -> usize {
        let mut len = 0;
        for iter in self.iterators.iter() {
            len += iter.len();
        }
        len
    }
}
