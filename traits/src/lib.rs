// Player struct representing a game player with various attributes
#[derive(Debug)]
pub struct Player {
    pub name: String,          // Player's name
    pub strength: f64,         // Player's strength level
    pub score: i32,            // Player's score
    pub money: i32,            // Player's money
    pub weapons: Vec<String>, // Player's list of weapons
}

// Fruit struct representing a type of food with weight
pub struct Fruit {
    pub weight_in_kg: f64, // Weight of the fruit in kilograms
}

// Meat struct representing another type of food with weight and fat content
pub struct Meat {
    pub weight_in_kg: f64, // Weight of the meat in kilograms
    pub fat_content: f64,  // Fat content of the meat
}

// Implementation block for Player
impl Player {
    // Method allowing the player to eat a food item and gain strength
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives(); // Increase strength based on the food's give value
    }
}

use std::fmt;

// Implement Display trait for Player to allow formatted output
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print player name
        writeln!(f, "{}", self.name)?;
        // Print player's strength, score, and money
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength, self.score, self.money
        )?;
        // Print player's weapons
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

// Trait representing any type of food that gives strength
pub trait Food {
    fn gives(&self) -> f64; // Returns how much strength the food gives
}

// Implement Food trait for Fruit
impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0 // Strength is 4 times the weight
    }
}

// Implement Food trait for Meat
impl Food for Meat {
    fn gives(&self) -> f64 {
        // Strength is 4 times the weight plus additional based on fat content
        self.weight_in_kg * 4.0 + self.weight_in_kg * self.fat_content * 5.0
    }
}

// Test module
#[cfg(test)]
mod test {
    use super::*;

    // Test for the gives method for both Fruit and Meat
    #[test]
    fn test_gives() {
        let apple = Fruit { weight_in_kg: 1.0 };
        assert_eq!(apple.gives(), 4.0);

        let steak = Meat {
            weight_in_kg: 1.0,
            fat_content: 1.0,
        };
        assert_eq!(steak.gives(), 9.0); // 4 + 5 = 9

        let steak = Meat {
            weight_in_kg: 1.0,
            fat_content: 0.0,
        };
        assert_eq!(steak.gives(), 4.0); // No additional strength from fat

        let steak = Meat {
            weight_in_kg: 1.5,
            fat_content: 0.3,
        };
        assert_eq!(steak.gives(), 8.25); // 1.5*4 + 1.5*0.3*5 = 6 + 2.25 = 8.25
    }

    // Test for Player eating food and strength increasing accordingly
    #[test]
    fn test_eat() {
        let apple = Fruit { weight_in_kg: 1.0 };
        assert_eq!(apple.gives(), 4.0);

        let steak = Meat {
            weight_in_kg: 1.0,
            fat_content: 1.0,
        };

        let mut player1 = Player {
            name: String::from("player1"),
            strength: 1.0,
            score: 0,
            money: 0,
            weapons: vec![String::from("knife"), String::from("shotgun")],
        };

        player1.eat(apple); // +4.0
        assert_eq!(player1.strength, 5.0);

        player1.eat(steak); // +9.0
        assert_eq!(player1.strength, 14.0);
    }

    // Test for Display trait implementation for Player
    #[test]
    fn test_display() {
        let player1 = Player {
            name: String::from("player1"),
            strength: 1.0,
            score: 0,
            money: 0,
            weapons: vec![String::from("knife"), String::from("shotgun")],
        };

        println!("{}", player1); // Print player info using Display trait

        assert_eq!(
            player1.to_string(), // Convert player to string
            "player1\nStrength: 1, Score: 0, Money: 0\nWeapons: [\"knife\", \"shotgun\"]"
        )
    }
}