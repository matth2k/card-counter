/*!

  A hand in blackjack

*/

use crate::card::Card;
use std::fmt::Display;

/// The value of a hand of cards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    /// A + 10
    Blackjack,
    /// The highest without going over 21, and num of hard aces (counted as 1)
    Soft(u8, u8),
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
            Value::Soft(v, _) | Value::Hard(v) => Some(v),
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
                self.val = Value::Soft(11, 0);
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
                    Value::Soft(*possible_vals.iter().max().unwrap(), 0)
                }
            }
            Value::Soft(v, a) => {
                let deductions = self.num_aces - a;
                let other_vals: Vec<(u8, u8)> = (0..(deductions + 1))
                    .map(|i| ((v - i * 10), a + i))
                    .collect();
                let mut all_vals = Vec::new();

                for (o, c) in card.values().iter().rev().enumerate() {
                    for (b, a) in &other_vals {
                        all_vals.push(((c + b), *a + o as u8));
                    }
                }

                if self.cards.len() == 1 && v == 11 && card.values()[0] == 10 {
                    Value::Blackjack
                } else if all_vals.iter().all(|(x, _)| *x > 21) {
                    Value::Bust
                } else if all_vals.iter().filter(|(x, _)| *x <= 21).count() == 1 {
                    Value::Hard(all_vals.iter().min().unwrap().0)
                } else {
                    let val = all_vals.iter().filter(|(x, _)| *x <= 21).max().unwrap();
                    Value::Soft(val.0, val.1)
                }
            }

            Value::Blackjack => {
                let other_vals: Vec<u8> = vec![11, 21];
                let mut all_vals = Vec::new();
                for a in card.values() {
                    for b in &other_vals {
                        all_vals.push(a + b);
                    }
                }

                if all_vals.iter().all(|x| *x > 21) {
                    Value::Bust
                } else {
                    Value::Hard(*all_vals.iter().min().unwrap())
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

    /// Returns true if the hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns true if the hand can be doubled (only 2 cards)
    pub fn can_double(&self) -> bool {
        self.cards.len() == 2
    }

    /// Returns true if the hand can be split
    pub fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].rank() == self.cards[1].rank()
    }

    /// Returns true if the hand is a soft hand (contains an ace counted as 11)
    pub fn is_soft(&self) -> bool {
        matches!(self.val, Value::Soft(_, _))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self.cards.iter().map(|c| format!("{}", c)).collect();
        let cards_str = cards.join(", ");
        match self.value() {
            Some(_v) => write!(f, "[{}] (Value: {:?})", cards_str, self.val),
            None => write!(f, "[{}] (Bust)", cards_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

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

    #[test]
    fn soft_hand_multiple_aces_deuces() {
        let mut hand = Hand::default();
        hand.insert(Card::new(Suit::Spade, Rank::Two));
        hand.insert(Card::new(Suit::Diamond, Rank::Ace));
        hand.insert(Card::new(Suit::Spade, Rank::Two));
        hand.insert(Card::new(Suit::Diamond, Rank::Ace));
        hand.insert(Card::new(Suit::Diamond, Rank::Ace));
        assert_eq!(hand.value(), Some(17));
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
    }
}
