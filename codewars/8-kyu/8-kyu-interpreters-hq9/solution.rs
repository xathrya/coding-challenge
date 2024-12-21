// https://www.codewars.com/kata/591588d49f4056e13f000001

// solution 1: 1279ms
fn hq9(code: &str) -> Option<String> {
    match code {
        "H" => Some("Hello World!".into()),
        "Q" => Some("Q".into()),
        "9" => {
            let mut result = String::new();
            
            result.push_str("99 bottles of beer on the wall, 99 bottles of beer.\n");

            for i in (2u8..99).rev() {
                result.push_str(&format!("Take one down and pass it around, {} bottles of beer on the wall.\n", i));
                result.push_str(&format!("{} bottles of beer on the wall, {} bottles of beer.\n", i, i));
            }

            result.push_str("Take one down and pass it around, 1 bottle of beer on the wall.\n");
            result.push_str("1 bottle of beer on the wall, 1 bottle of beer.\n");
            result.push_str("Take one down and pass it around, no more bottles of beer on the wall.\n");
            result.push_str("No more bottles of beer on the wall, no more bottles of beer.\n");
            result.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.");

            Some(result)
        },
        _   => None,
    }
}