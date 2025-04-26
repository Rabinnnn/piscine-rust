pub fn first_fifty_even_square() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 2..102 {
        if i % 2 == 0 {
            v.push(i * i);
        }
    }
    v
}