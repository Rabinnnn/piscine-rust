pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    if index >= slice.len() {
        panic!("Index out of bounds");
    }
    &slice[index]
}

