// A generic function that returns the value it receives
pub fn identity<T>(v: T) -> T {
    v
}

#[cfg(test)]
mod test {
    use super::*;

    // A simple struct used for testing the identity function with custom types
    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Test identity with an integer
    #[test]
    fn test_with_int() {
        assert_eq!(identity(3), 3);
    }

    // Test identity with a floating point number
    #[test]
    fn test_with_float() {
        assert_eq!(identity(1.0), 1.0);
    }

    // Test identity with a string slice
    #[test]
    fn test_with_str() {
        assert_eq!(identity("you"), "you");
    }

    // Test identity with a reference to a custom struct
    #[test]
    fn test_with_struct() {
        let s = Point { x: 1, y: 2 };
        assert_eq!(identity(&s), &s);
    }
}