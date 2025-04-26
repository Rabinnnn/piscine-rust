pub use crate::RomanDigit::*;
use roman_numbers_iter::{RomanDigit, RomanNumber};

fn main() {
    let mut number = RomanNumber::from(15);

    println!("{:?}", number);
    println!("{:?}", number.next());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_numbers_iterator_test() {
        assert_eq!(
            RomanNumber::from(1).0,
            RomanNumber::from(0).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(9).0,
            RomanNumber::from(8).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(6).0,
            RomanNumber::from(5).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(14).0,
            RomanNumber::from(13).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(34).0,
            RomanNumber::from(33).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(50).0,
            RomanNumber::from(49).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(200).0,
            RomanNumber::from(199).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(500).0,
            RomanNumber::from(499).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(1533).0,
            RomanNumber::from(1532).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(2349).0,
            RomanNumber::from(2348).next().unwrap().0
        );
    }
}