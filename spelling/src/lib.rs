pub fn spell(n: u64) -> String {
    // Dispatch spelling based on the number's magnitude
    match n {
        0..=99 => spells_below_100(n),        // Spell numbers from 0 to 99
        100..=999 => spells_hundreds(n),      // Spell numbers from 100 to 999
        _ => spells_bignum(n),                // Spell numbers 1000 and above
    }
}

pub fn spells_below_100(n: u64) -> String {
    // Return English spelling for numbers less than 100
    match n {
        0 => "zero".to_string(),              // Special case for 0
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => {
            // Handle numbers between tens (e.g., 21–99)
            let rem = n % 10; // Units place
            format!("{}-{}", spells_below_100(n - rem), spells_below_100(rem))
        }
    }
}

pub fn spells_hundreds(n: u64) -> String {
    // Spell numbers from 100 to 999
    let div = n / 100;                       // Hundreds digit
    let rem = n % 100;                       // Remainder
    let mut enc_str = format!("{} hundred", spells_below_100(div)); // "<digit> hundred"
    if rem != 0 {
        // Add remainder if not zero
        enc_str = format!("{} {}", enc_str, spells_below_100(rem));
    }
    enc_str
}

pub fn spells_bignum(n: u64) -> String {
    // Spell large numbers using chunks of three digits
    let mut enc_chunks: Vec<String> = vec![];   // Store word parts
    let mut chunks: Vec<u64> = vec![0; 7];      // Up to quintillions
    let mut m = n;
    for e in chunks.iter_mut() {
        // Break number into 3-digit chunks
        let rem = m % 1_000;
        m = m / 1_000;
        *e += rem;
    }
    for (idx, chunk) in chunks.into_iter().enumerate() {
        // Iterate through chunks and spell them
        let substr = match idx {
            0 => "",                            // Base
            1 => "thousand",
            2 => "million",
            3 => "billion",
            4 => "trillion",
            5 => "quadrillion",
            _ => "quintillion",                 // Fallback
        };
        if chunk != 0 {
            // Only spell non-zero chunks
            enc_chunks.push(
                format!("{} {}", spell(chunk), substr)
                    .trim()
                    .to_string(),
            );
        }
    }
    enc_chunks.reverse();                       // Most significant chunk first
    enc_chunks.join(" ")                        // Join all parts with space
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(spell(0), String::from("zero"));
        assert_eq!(spell(1), String::from("one"));
        assert_eq!(spell(14), String::from("fourteen"));
        assert_eq!(spell(20), String::from("twenty"));
        assert_eq!(spell(22), String::from("twenty-two"));
        assert_eq!(spell(101), String::from("one hundred one"));
        assert_eq!(spell(120), String::from("one hundred twenty"));
        assert_eq!(spell(123), String::from("one hundred twenty-three"));
        assert_eq!(spell(1000), String::from("one thousand"));
        assert_eq!(spell(1055), String::from("one thousand fifty-five"));
        assert_eq!(
            spell(1234),
            String::from("one thousand two hundred thirty-four")
        );
        assert_eq!(
            spell(10123),
            String::from("ten thousand one hundred twenty-three")
        );
        assert_eq!(
            spell(910112),
            String::from("nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            spell(651123),
            String::from("six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(spell(810000), String::from("eight hundred ten thousand"));
        assert_eq!(spell(1000000), String::from("one million"));
    }
}