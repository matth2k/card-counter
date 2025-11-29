/*!

  A shoe from which to deal cards

*/

use crate::card::{Card, Rank, Suit};
use rand::{rng, seq::SliceRandom};
use std::ops::Index;

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
    /// Returns the number of cards in the shoe
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if the shoe is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Shuffle the shoe
    fn shuffle(&mut self) {
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

    /// Returns the number of decks loaded into the shoe
    pub fn num_decks(&self) -> usize {
        self.decks
    }

    /// Return the running count
    pub fn running_count(&self) -> f32 {
        (self.count as f32) / (self.decks as f32)
    }

    /// Returns how far the deck has been penetrated
    pub fn penetration(&self) -> f32 {
        1.0 - (self.cards.len() as f32) / ((self.decks * 52) as f32)
    }
}
