// https://www.codewars.com/kata/55e2adece53b4cdcb900006c

// solution 1: 1367ms
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }

    let full_time = (g * 3600) / (v2 - v1);
    let second = full_time % 60;
    let hour   = full_time / 3600;
    let minute = (full_time % 3600) / 60;

    Some(vec![hour, minute, second])
}