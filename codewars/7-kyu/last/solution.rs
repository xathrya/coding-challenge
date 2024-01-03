// https://www.codewars.com/kata/541629460b198da04e000bb9

// solution 1: 1123ms
fn last<T: Clone>(slice: &[T]) -> T {
    slice.last().unwrap().clone()
}

//
fn last<T: Clone>(slice: &[T]) -> T {
    let l = slice.last();
    match l {
        None => panic!("empty"),
        Some(x) => x.clone(),
    }
}