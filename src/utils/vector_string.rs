pub fn vector_string(vec: Vec<&str>) -> Vec<String> {
    vec.into_iter().map(String::from).collect()
}
