// https://www.codewars.com/kata/542ebbdb494db239f8000046

// solution 1: 1313ms 
fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    let mut iter = slice.into_iter();
    while let Some(next) = iter.next() {
        if next == &find { return iter.next().cloned(); }
    }
    None
}