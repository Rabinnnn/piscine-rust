pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    // Handle empty array case
    if arr.is_empty() {
        return Vec::new();
    }

    // Handle single element case
    if arr.len() == 1 {
        return vec![];
    }

    let mut result = Vec::new();
    for i in 0..arr.len() {
        let mut product = 1;
        for j in 0..arr.len() {
            if i != j {
                product *= arr[j];
            }
        }
        result.push(product);
    }
    result
}