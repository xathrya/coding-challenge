// https://www.codewars.com/kata/53af2b8861023f1d88000832

// solution 1: 1430ms
fn are_you_playing_banjo(name: &str) -> String {
    let verdict = if name.starts_with('R') || name.starts_with('r') {
        "plays banjo"
    } else {
        "does not play banjo"
    };
    
    format!("{} {}", name, verdict)
}


// solution 2: 1771ms
fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r"   => format!("{} plays banjo", name),
        _           => format!("{} does not play banjo", name),
    }
}