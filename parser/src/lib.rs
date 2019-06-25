#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

// Fn(Input) -> Result<(Input, Output), Error>
// Fn(Input) -> Result<(&str, Element), &str>

// take string and return result of pair(&str, ()) or the error type &str
fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

// parser builder
// function that produces a parser for a static string of any length
fn match_literal(expexted: &'static str) -> impl Fn(&str) -> Result<(&str, ()),
                                                                    &str> {

}
