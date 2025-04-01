pub fn fahrenheit_to_celsius(f: f64) -> f64 {
  let output =  (f - 32.0) * 5.0 / 9.0;
  output + 0.000 000 000 000 001
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}