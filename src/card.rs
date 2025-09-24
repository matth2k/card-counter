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

    /// Returns true if card is an ace
    pub fn is_ace(&self) -> bool {
        self.rank == Rank::Ace
    }
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

/// The value of a hand of cards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    /// A + 10
    Blackjack,
    /// The highest without going over 21
    Soft(u8),
    /// A hard value
    Hard(u8),
    /// Bust
    Bust,
}

impl Value {
    /// Get the numeric value of the hand.
    /// Returns None if the hand is bust.
    fn value(self) -> Option<u8> {
        match self {
            Value::Blackjack => Some(21),
            Value::Soft(v) | Value::Hard(v) => Some(v),
            Value::Bust => None,
        }
    }
}

/// A hand of blackjack
/// The value of a hand of cards
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    num_aces: u8,
    val: Value,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: Vec::new(),
            num_aces: 0,
            val: Value::Hard(0),
        }
    }
}

impl Hand {
    /// Deal a card to the hand
    pub fn insert(&mut self, card: Card) {
        if self.cards.is_empty() {
            if card.is_ace() {
                self.val = Value::Soft(11);
                self.num_aces += 1;
            } else {
                self.val = Value::Hard(card.values()[0]);
            }
            self.cards.push(card);
            return;
        }

        self.val = match self.val {
            Value::Bust => Value::Bust,
            Value::Hard(v) => {
                let possible_vals: Vec<u8> = card.values().iter().map(|&x| x + v).collect();
                if self.cards.len() == 1 && v == 10 && card.is_ace() {
                    Value::Blackjack
                } else if possible_vals.iter().all(|x| *x > 21) {
                    Value::Bust
                } else if possible_vals.len() == 1 {
                    Value::Hard(possible_vals[0])
                } else if possible_vals.iter().any(|x| *x > 21) {
                    Value::Hard(*possible_vals.iter().min().unwrap())
                } else {
                    Value::Soft(*possible_vals.iter().max().unwrap())
                }
            }
            Value::Soft(v) => {
                let card_vals: Vec<u8> = card.values().to_vec();
                // TODO: Check underflow
                let other_vals: Vec<u8> = (0..self.num_aces)
                    .map(|i| v - i * 10)
                    .filter(|x| *x <= 21)
                    .collect();
                let mut all_vals = Vec::new();
                for a in &card_vals {
                    for b in &other_vals {
                        all_vals.push(a + b);
                    }
                }
                if v == 11 && card_vals[0] == 10 {
                    Value::Blackjack
                } else if all_vals.iter().all(|x| *x > 21) {
                    Value::Bust
                } else if all_vals.iter().filter(|x| **x <= 21).count() == 1 {
                    Value::Hard(*all_vals.iter().min().unwrap())
                } else {
                    Value::Soft(*all_vals.iter().filter(|x| **x <= 21).max().unwrap())
                }
            }

            Value::Blackjack => {
                let card_vals: Vec<u8> = card.values().to_vec();
                let other_vals: Vec<u8> = vec![11, 21];
                let mut all_vals = Vec::new();
                for a in &card_vals {
                    for b in &other_vals {
                        all_vals.push(a + b);
                    }
                }
                if all_vals.iter().all(|x| *x > 21) {
                    Value::Bust
                } else if all_vals.iter().filter(|x| **x <= 21).count() == 1 {
                    Value::Hard(*all_vals.iter().min().unwrap())
                } else {
                    Value::Soft(*all_vals.iter().filter(|x| **x <= 21).max().unwrap())
                }
            }
        };

        if card.is_ace() {
            self.num_aces += 1;
        }

        self.cards.push(card);
    }

    /// Get the value of the hand
    pub fn value(&self) -> Option<u8> {
        self.val.value()
    }

    /// Returns true if the hand is a bust
    pub fn is_bust(&self) -> bool {
        self.val == Value::Bust
    }

    /// Returns true if the hand is a blackjack
    pub fn is_blackjack(&self) -> bool {
        self.val == Value::Blackjack
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self.cards.iter().map(|c| format!("{}", c)).collect();
        let cards_str = cards.join(", ");
        match self.value() {
            Some(v) => write!(f, "[{}] (Value: {})", cards_str, v),
            None => write!(f, "[{}] (Bust)", cards_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_1() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Heart, Rank::Ace));
        hand.insert(Card::new(Suit::Heart, Rank::Four));
        assert_eq!(hand.value(), Some(15));
        hand.insert(Card::new(Suit::Heart, Rank::Ace));
        assert_eq!(hand.value(), Some(16));
    }

    #[test]
    fn hard_hand_no_ace() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Spade, Rank::Ten));
        hand.insert(Card::new(Suit::Diamond, Rank::Seven));
        assert_eq!(hand.value(), Some(17));
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
    }

    #[test]
    fn hard_hand_bust() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Club, Rank::Ten));
        hand.insert(Card::new(Suit::Heart, Rank::Nine));
        hand.insert(Card::new(Suit::Diamond, Rank::Five));
        assert_eq!(hand.value(), None);
        assert!(hand.is_bust());
    }

    #[test]
    fn hard_hand_exact_21() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Spade, Rank::Ten));
        hand.insert(Card::new(Suit::Diamond, Rank::Six));
        hand.insert(Card::new(Suit::Club, Rank::Five));
        assert_eq!(hand.value(), Some(21));
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
    }

    #[test]
    fn soft_hand_ace_and_six() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Spade, Rank::Ace));
        hand.insert(Card::new(Suit::Diamond, Rank::Six));
        assert_eq!(hand.value(), Some(17));
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
    }

    #[test]
    fn soft_hand_multiple_aces() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Spade, Rank::Ace));
        hand.insert(Card::new(Suit::Club, Rank::Eight));
        hand.insert(Card::new(Suit::Diamond, Rank::Ace));
        assert_eq!(hand.value(), Some(20));
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
    }

    #[test]
    fn soft_hand_ace_and_face_card() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Spade, Rank::Ace));
        hand.insert(Card::new(Suit::Diamond, Rank::King));
        assert_eq!(hand.value(), Some(21));
        assert!(hand.is_blackjack());
        assert!(!hand.is_bust());
    }
}
