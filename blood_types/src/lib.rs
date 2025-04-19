#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,    // Antigen A
    AB,   // Antigen AB
    B,    // Antigen B
    O,    // Antigen O (universal donor)
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),     // String "A" maps to Antigen::A
            "AB" => Ok(Antigen::AB),   // String "AB" maps to Antigen::AB
            "B" => Ok(Antigen::B),     // String "B" maps to Antigen::B
            "O" => Ok(Antigen::O),     // String "O" maps to Antigen::O
            other => Err(format!("`{}` is not a valid antigen", other)), // Invalid input
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive, // Rh+
    Negative, // Rh-
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(rhf: &str) -> Result<Self, Self::Err> {
        match rhf {
            "+" => Ok(RhFactor::Positive), // "+" maps to Positive
            "-" => Ok(RhFactor::Negative), // "-" maps to Negative
            o => Err(format!("`{}` is not a valid Rh Factor", o)), // Invalid input
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,       // ABO antigen component
    pub rh_factor: RhFactor,    // Rh factor component
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rh_factor == other.rh_factor {
            return self.antigen.cmp(&other.antigen); // If Rh factors are equal, compare antigens
        }
        self.antigen.cmp(&other.antigen) // Otherwise still compare antigens (ignores Rh)
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(bt: &str) -> Result<Self, Self::Err> {
        // Validate blood type string length (min 2, max 3)
        if bt.len() > 3 || bt.len() < 2 {
            return Err(format!(
                "Invalid antigen: `{}` invalid length: {}",
                bt,
                bt.len()
            ));
        }

        // Extract last character as Rh factor
        let rh_fac_str = bt.get(bt.len() - 1..);

        if let None = rh_fac_str {
            return Err(format!("Invalid suffix {:?}", rh_fac_str)); // Check for missing Rh suffix
        }

        // Parse Rh factor and antigen
        let rh_factor = rh_fac_str.unwrap().parse()?;
        let antigen = bt.get(..bt.len() - 1).unwrap().parse()?;

        Ok(BloodType { antigen, rh_factor }) // Return parsed blood type
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.antigen)?; // Print antigen

        if self.rh_factor == RhFactor::Positive {
            return write!(f, "+"); // Append "+" if Rh positive
        }

        write!(f, "-") // Append "-" if Rh negative
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Negative cannot receive from positive
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }

        // Universal donor check: can receive from O
        if other.antigen == Antigen::O {
            return true;
        }

        // AB can receive from any antigen, otherwise must match
        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    pub fn donors(&self) -> Vec<Self> {
        // Generate all compatible donor blood types
        let mut blood_types = Vec::new();

        // If antigen is O, only accept O; otherwise accept O and matching antigen
        let mut antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        // Rh negative can receive only negative; positive can receive both
        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        // AB can receive from A and B as well
        if self.antigen == Antigen::AB {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        // Generate combinations of compatible antigens and Rh factors
        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        // Generate all compatible recipient blood types
        let mut blood_types = Vec::new();

        // If not AB, valid recipients include AB and self antigen; AB can only go to AB
        let mut antigens = if self.antigen != Antigen::AB {
            vec![Antigen::AB, self.antigen.clone()]
        } else {
            vec![Antigen::AB]
        };

        // Rh negative can give to both; Rh positive can only give to positive
        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Positive, RhFactor::Negative]
        } else {
            vec![RhFactor::Positive]
        };

        // O can also be given to A and B
        if self.antigen == Antigen::O {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        // Generate combinations of valid recipients
        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "A+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type: BloodType = "A-".parse().unwrap();
        let other_bt: BloodType = "A+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "A-".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "O+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type: BloodType = "AB+".parse().unwrap();
        let other_bt: BloodType = "O+".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "O-".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood = "AB+";
        let blood_type: BloodType = blood.parse().unwrap();
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood = "A-";
        let blood_type = blood.parse::<BloodType>().unwrap();
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    #[should_panic]
    fn test_unexistent_blood_type() {
        let _blood_type: BloodType = "AO-".parse().unwrap();
    }

    #[test]
    fn test_donors() {
        let mut givers = "AB+".parse::<BloodType>().unwrap().donors();
        println!("Before sorting {:?}", &givers);
        givers.sort();
        println!("{:?}", &givers);
        let mut expected = vec![
            "AB-".parse::<BloodType>().unwrap(),
            "A-".parse().unwrap(),
            "B-".parse().unwrap(),
            "O-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "B+".parse().unwrap(),
            "O+".parse().unwrap(),
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let mut givers = "A-".parse::<BloodType>().unwrap().donors();
        givers.sort();
        let mut expected: Vec<BloodType> = vec!["A-".parse().unwrap(), "O-".parse().unwrap()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let mut givers = "O-".parse::<BloodType>().unwrap().donors();
        givers.sort();
        let mut expected: Vec<BloodType> = vec!["O-".parse().unwrap()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let mut recipients: Vec<BloodType> = "AB+".parse::<BloodType>().unwrap().recipients();
        recipients.sort();
        let mut expected: Vec<BloodType> = vec!["AB+".parse().unwrap()];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let mut recipients = "A-".parse::<BloodType>().unwrap().recipients();
        recipients.sort();
        let mut expected: Vec<BloodType> = vec![
            "A-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "AB-".parse().unwrap(),
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_output() {
        let blood_type: BloodType = "O+".parse().unwrap();
        println!("recipients of O+ {:?}", blood_type.recipients());
        println!("donors of O+ {:?}", blood_type.donors());
        let another_blood_type: BloodType = "A-".parse().unwrap();
        println!(
            "donors of O+ can receive from {:?} {:?}",
            &another_blood_type,
            blood_type.can_receive_from(&another_blood_type)
        );
    }
}