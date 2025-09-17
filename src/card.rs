/*!

  Cards

*/

use rand::{rng, seq::SliceRandom};
use std::{fmt::Display, ops::Index};

/// Suit of a card
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Suit {
    /// Heart
    Heart,
    /// Diamond
    Diamond,
    /// Club
    Club,
    /// Spade
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Heart => write!(f, "♡"),
            Self::Diamond => write!(f, "♢"),
            Self::Club => write!(f, "♧"),
            Self::Spade => write!(f, "♤"),
        }
    }
}

/// Rank of a card
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Rank {
    /// 1/11
    Ace,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
    /// 8
    Eight,
    /// 9
    Nine,
    /// 10
    Ten,
    /// 10
    Jack,
    /// 10
    Queen,
    /// 10
    King,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ace => write!(f, "A"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "10"),
            Self::Jack => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
        }
    }
}

/// A playing card
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    /// Create a new card
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    /// Get the Suit of the card
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Get the rank of the card
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// Get the value of the card
    pub fn values(&self) -> &[u8] {
        match self.rank {
            Rank::Ace => &[1, 11],
            Rank::Two => &[2],
            Rank::Three => &[3],
            Rank::Four => &[4],
            Rank::Five => &[5],
            Rank::Six => &[6],
            Rank::Seven => &[7],
            Rank::Eight => &[8],
            Rank::Nine => &[9],
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => &[10],
        }
    }

    /// Return the count of a card
    pub fn count(&self) -> i8 {
        match self.rank {
            Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six => 1,
            Rank::Seven | Rank::Eight | Rank::Nine => 0,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace => -1,
        }
    }
}

/// The value of a hand of cards
pub enum Hand {
    /// A + 10
    Blackjack,
    /// A soft value
    Soft(u8),
    /// A hard value
    Hard(u8),
}

/// A shoe of cards
pub struct Shoe {
    /// the cards
    cards: Vec<Card>,
    /// the running count
    count: i32,
    /// the number of decks
    decks: usize,
}

impl Shoe {
    /// Create a new shoe with the given number of decks
    pub fn new(decks: usize) -> Self {
        let mut cards = Vec::new();

        for &s in &[Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            for &r in &[
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                for _ in 0..decks {
                    cards.push(Card::new(s, r));
                }
            }
        }

        let mut shoe = Self {
            cards,
            count: 0,
            decks,
        };
        shoe.shuffle();
        shoe
    }
}

impl Index<usize> for Shoe {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl Shoe {
    /// Get the number of cards in the shoe
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Is the shoe empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Shuffle the shoe
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deal a card from the shoe
    pub fn deal(&mut self) -> Option<Card> {
        if let Some(card) = self.cards.pop() {
            self.count += card.count() as i32;
            Some(card)
        } else {
            None
        }
    }

    /// Return the running count
    pub fn running_count(&self) -> f32 {
        (self.count as f32) / (self.decks as f32)
    }
}
