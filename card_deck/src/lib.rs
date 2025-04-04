Instructions
A standard deck of cards has 52 cards: 4 suits with 13 cards per suit. Represent the cards from a deck:

Create an enum to represent the Suit.
Implement the associated function random, which returns a random Suit (Heart, Diamond, Spade or Club).
Create a Rank enum. For ace and face cards, it can be one of Ace, King, Queen or Jack. For the values from 2 to 10, it can have a Number value associated to a u8.
Create an associated function to Rank called Random that returns a random Rank.
Create a structure name Card which has the fields suit and rank.
Define:

The associated function translate for Rank and Suit:
For Suit, translate converts an integer value (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
For Rank, translate converts an integer value (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
The associated function random for Rank and Suit returns a random Rank and Suit respectively.
Finally define the function winner_card which returns true if the card passed as an argument is an ace of spades.
Expected Functions and Structures
pub enum Suit {
}

pub enum Rank {
}

impl Suit {
    pub fn random() -> Suit {
    }

    pub fn translate(value: u8) -> Suit {
    }
}

impl Rank {
    pub fn random() -> Rank {
    }

    pub fn translate(value: u8) -> Rank {
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
}use rand::Rng;

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
