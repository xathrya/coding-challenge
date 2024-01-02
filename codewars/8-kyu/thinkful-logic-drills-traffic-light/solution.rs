// https://www.codewars.com/kata/58649884a1659ed6cb000072

// solution 1: 1263ms
fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow"=> "red",
        "red"   => "green",
        _       => todo!(),
    }.to_string()
}