// https://www.codewars.com/kata/5c8bfa44b9d1192e1ebd3d15

// solution 1: 1379ms
fn warn_the_sheep(queue: &[&str]) -> String {
    let wolf_pos = queue
        .iter()
        .position(|x| x.bytes().next() == Some(b'w'))
        .unwrap();

    match queue.len() - wolf_pos -1 {
        0   => "Pls go away and stop eating my sheep".into(),
        n   => format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", n),
    }
}


// solution 2: 1674ms
fn warn_the_sheep(queue: &[&str]) -> String {
    match queue.iter().rev().position(|&a| a == "wolf").unwrap() {
        0   => "Pls go away and stop eating my sheep".into(),
        n   => format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", n),
    }
}