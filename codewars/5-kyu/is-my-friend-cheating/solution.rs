// https://www.codewars.com/kata/5547cc7dcad755e480000004

// solution 1: 1669ms
fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let m = m as u64;
    let total = m * (m + 1) / 2;
    let min_a = ((m - 1) * m / 2) / (m + 1);
    let max_a = f64::sqrt(total as f64 + 1.0) as u64;
    for a in min_a..max_a {
        let b = (total - a) / (a + 1);
        if a * b + a + b == total {
            result.push((a as i32, b as i32));
            result.push((b as i32, a as i32));
        }
    }
    result.sort_by(|a, b| a.cmp(b));
    result
}


// solution 2: 1402ms
fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let m = m as i64;
    let total = m * (m + 1) / 2;
    ((total - m)/(m + 1) .. m)
        .filter(|i| (total - i) % (i + 1) == 0)
        .map(|i| (i, (total - i) / (i + 1)))
        .map(|(a, b)| (a as i32, b as i32))
        .collect()
}