// https://www.codewars.com/kata/517abf86da9663f1d2000003

// solution 1: 2659ms
fn to_camel_case(text: &str) -> String {
    if text.len() == 0 {
        return "".to_string();
    }
    text.split(|v: char| !v.is_ascii_alphabetic())
        .filter(|w| w.len() > 0)
        .enumerate()
        .map(|(i, w)| if i == 0 { save_case(w) } else { title(w) })
        .collect()
}

fn save_case(t: &str) -> String {
    let first = t.chars().next().unwrap();
    if first.is_lowercase() {
        t.to_lowercase()
    } else {
        title(t)
    }
}

fn title(t: &str) -> String {
    t   .chars()
        .enumerate()
        .map(|(i, x)| {
            if i == 0 {
                x.to_ascii_uppercase()
            } else {
                x.to_ascii_lowercase()
            }
        })
        .collect()
}