// https://www.codewars.com/kata/582cb0224e56e068d800003c

// solution 1: 1155ms
fn litres(time: f64) -> i32 {
    (time * 0.5).floor() as i32
}