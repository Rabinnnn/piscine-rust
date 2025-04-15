// lib.rs or main.rs (depending on your setup)

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums: Vec<u32> = s
        .split_whitespace()
        .filter_map(|part| {
            if let Some(stripped) = part.strip_suffix('k') {
                stripped.parse::<f32>().ok().map(|n| (n * 1000.0) as u32)
            } else {
                part.parse::<u32>().ok()
            }
        })
        .collect();

    Box::new(nums)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
