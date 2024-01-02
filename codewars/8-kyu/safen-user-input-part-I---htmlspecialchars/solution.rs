// https://www.codewars.com/kata/56bcaedfcf6b7f2125001118

// solution 1: 1466ms
fn html_special_chars(html: &str) -> String {
    html.chars().fold(String::new(), |acc, c| match c {
        '<' => format!("{}&lt;", acc),
        '>' => format!("{}&gt;", acc),
        '"' => format!("{}&quot;", acc),
        '&' => format!("{}&amp;", acc),
        _ => format!("{}{}", acc, c),
    })
}

// solution 2: 1461ms
fn html_special_chars(html: &str) -> String {
    html.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}