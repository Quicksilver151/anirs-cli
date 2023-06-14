pub mod data;
pub mod download;
pub mod updates;
pub use data::*;
pub use download::*;
pub use updates::*;


// Random useful stuff:
pub fn separator(input: &str, separator: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for c in input.chars() {
        if c == separator {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            result.push(c.to_string());
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}
