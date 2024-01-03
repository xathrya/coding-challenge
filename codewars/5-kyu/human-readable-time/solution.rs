// https://www.codewars.com/kata/52685f7382004e774f0001f7

// option 1: 1323ms
fn make_readable(seconds: u32) -> String {
    let hours, seconds = div_mod(seconds, 3600);
    let minutes, seconds = div_mod(seconds, 60);
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn div_mod<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let q = x / y;
    let r = x % y;
    (q, r)
}

// option 2: 
fn make_readable(seconds: u32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    let hours   = minutes / 60;
    let minutes = minutes % 60

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}