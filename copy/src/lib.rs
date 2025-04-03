use std::f64::consts::E;
use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = E.powf(c as f64);
    let log_value = (c.abs() as f64).ln();
    (c, exp_value, log_value)
}

pub fn str_function(a: String) -> (String, String) {
    let mut exp_string = String::new();

    for ch in a.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let exp_value = E.powf(digit as f64);
            exp_string.push_str(&format!("{:.16} ", exp_value));  // Full precision
        } else {
            continue;
        }
    }

    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log_values: Vec<f64> = b.iter()
        .map(|&value| (value.abs() as f64).ln())
        .collect();
    (b, log_values)
}
