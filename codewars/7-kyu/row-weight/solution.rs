// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9

// solution 1: 1341ms
fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let mut weights = (0, 0);
    for (i, weight) in array.iter().enumerate() {
        if i % 2 == 0 {
            weights.0 += weight;
        } else {
            weights.1 += weight;
        }
    }
    weights
}
