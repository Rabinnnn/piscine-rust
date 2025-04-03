use std::f64::consts::E;
use std::f64;

pub fn nbr_function(value: f64) -> (f64, f64, f64) {
    // Exponential function: e^value
    let exp_value = E.powf(value);

    // Natural logarithm of the absolute value: ln(|value|)
    let log_value = value.abs().ln();

    // Return the tuple
    (value, exp_value, log_value)
}



pub fn str_function(value: String) -> (String, String) {
    // Create a result string to hold the exponential values
    let mut exp_string = String::new();

    // For each character, convert to a number, compute the exponential, and append it
    for ch in value.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let exp_value = E.powf(digit as f64);
            exp_string.push_str(&format!("{:.2} ", exp_value));
        }
    }

    // Return the original value and the exponential values
    (value, exp_string)
}


pub fn vec_function(values: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    // Borrow the `values` vector and use `iter()` to iterate over it
    let log_values: Vec<f64> = values.iter()
        .map(|&value| value.abs().ln()) // Calculate ln(|value|)
        .collect();

    // Return the original vector (ownership is preserved) and the transformed vector
    (values, log_values)
}