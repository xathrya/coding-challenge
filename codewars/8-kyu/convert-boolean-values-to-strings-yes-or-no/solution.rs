// https://www.codewars.com/kata/53369039d7ab3ac506000467

// solution 1: 1210ms
fn bool_to_word(value: bool) -> &'static str {
    match value {
        true    => "Yes",
        false   => "No",
    }
}