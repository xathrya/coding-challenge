// https://www.codewars.com/kata/586c1cf4b98de0399300001d

// 1142ms
fn combat(health: f32, damage: f32) -> f32 {
    (health - damage).max(0.0)
}