#![cfg(test)]

pub mod easycrypt;

///
/// Compare two strings without taking formatting into account.
///
pub fn equals_modulo_formatting(s1: &str, s2: &str) -> bool {
    let cleaned_s1: String = s1.chars().filter(|c| !c.is_whitespace()).collect();
    let cleaned_s2: String = s2.chars().filter(|c| !c.is_whitespace()).collect();
    cleaned_s1 == cleaned_s2
}
