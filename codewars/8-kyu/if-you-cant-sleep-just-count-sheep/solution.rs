// https://www.codewars.com/kata/if-you-cant-sleep-just-count-sheep

// solution 1: 1600ms
fn count_sheep(n: u32) -> String {
    (1..=n).map(|i| format!("{} sheep...", i)).collect()
}