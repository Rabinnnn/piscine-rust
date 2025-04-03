use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = f64::exp(c as f64); // Exponential of the value
    let ln_value = if c == 0 { f64::NEG_INFINITY } else { f64::ln(c.abs() as f64) }; // Natural log of the absolute value, handle 0 case
    (c, exp_value, ln_value)
}

pub fn str_function(a: String) -> (String, String) {
    let values: Vec<f64> = a
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|n| f64::exp(n as f64)) // Exponential of each number
        .collect();

    let exp_string = values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" "); // Join the exp results into a single string

    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b
        .iter()
        .map(|&x| f64::ln(x.abs() as f64)) // Log of the absolute value of each element
        .collect();

    (b, ln_values)
}