use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    // Returns a random Suit
    pub fn random() -> Suit {
        let value = rand::thread_rng().gen_range(1..=4);
        Suit::translate(value)
    }

    // Translates a u8 into a Suit (1-4)
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value: must be between 1 and 4"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8), // 2 to 10
    Jack,
    Queen,
    King,
}

impl Rank {
    // Returns a random Rank
    pub fn random() -> Rank {
        let value = rand::thread_rng().gen_range(1..=13);
        Rank::translate(value)
    }

    // Translates a u8 into a Rank (1-13)
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value: must be between 1 and 13"),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// Returns true if the card is Ace of Spades
pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
