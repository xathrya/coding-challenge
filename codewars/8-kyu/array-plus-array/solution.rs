// https://www.codewars.com/kata/5a2be17aee1aaefe2a000151

// solution 1: 1375ms
fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().chain(ys).sum()
}


// solution 2:
fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().sum::<i32>() + ys.iter().sum::<i32>()
}