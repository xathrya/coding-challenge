// https://www.codewars.com/kata/54edbc7200b811e956000556

// solution 1: 1410ms
fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&b| b).count() as u8
}