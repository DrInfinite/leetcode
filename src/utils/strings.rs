#![allow(dead_code)]
pub struct Strings;

impl Strings {
    pub fn vector_string(vec: Vec<&str>) -> Vec<String> {
        vec.into_iter().map(|s| s.to_string()).collect()
    }
}
