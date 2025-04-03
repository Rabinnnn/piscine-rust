use std::f64::consts::E;
use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    // Exponential function: e^value
    let exp_value = E.powf(c as f64); // Convert i32 to f64 for the exponential function

    // Natural logarithm of the absolute value: ln(|value|)
    let log_value = (c.abs() as f64).ln(); // Convert i32 to f64 for the logarithm

    // Return the tuple with i32 and f64 values
    (c, exp_value, log_value)
}

pub fn str_function(a: String) -> (String, String) {
    let mut exp_string = String::new();

    // For each character, convert to a number, compute the exponential, and append it
    for ch in a.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let exp_value = E.powf(digit as f64); // Exponential of the digit
            exp_string.push_str(&format!("{:.2} ", exp_value));
        } else {
            continue;  // Skipping non-digit characters
        }
    }

    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    // Map through the values in `b`, calculate ln(|value|) for each i32, and collect into a Vec<f64>
    let log_values: Vec<f64> = b.iter()
        .map(|&value| (value.abs() as f64).ln()) // Convert i32 to f64 for logarithmic calculation
        .collect();

    // Return the original Vec<i32> and the new Vec<f64>
    (b, log_values)
}
