/*!

  Abstract the Blackjack table

*/

use std::fmt::Display;

use crate::{hand::Hand, shoe::Shoe};

/// Represents the outcome of a bet
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Outcome {
    /// Pays 3:2 or 6:5
    Blackjack,
    /// Pays 1:1
    Win,
    /// A loss
    Lose,
    /// A push
    Push,
}

/// A blackjack table
pub struct Table {
    /// The dealer's hand
    dealer: Hand,
    /// The players' hands
    players: Vec<Hand>,
    /// The shoe
    shoe: Shoe,
    /// The deck penetration before reshuffle
    penetration: f32,
    /// The number of decks to play with
    num_decks: usize,
}

impl Table {
    /// Creates a new blackjack table
    pub fn new(num_decks: usize, num_players: usize) -> Self {
        let shoe = Shoe::new(num_decks);
        let players = vec![Hand::default(); num_players];
        let dealer = Hand::default();

        Self {
            dealer,
            players,
            shoe,
            penetration: 0.75,
            num_decks,
        }
    }

    /// Resets the table
    pub fn reset(&mut self) {
        self.shoe = Shoe::new(self.num_decks);
        self.dealer = Hand::default();
        for player in &mut self.players {
            *player = Hand::default();
        }
    }

    /// Clear the current hands
    pub fn clear_hands(&mut self) {
        self.dealer = Hand::default();
        for player in &mut self.players {
            *player = Hand::default();
        }
    }

    /// Deal the initial hands
    pub fn deal(&mut self) {
        assert!(self.dealer.is_empty());
        for player in &self.players {
            assert!(player.is_empty());
        }

        if self.shoe.get_penetration() > self.penetration {
            self.shoe = Shoe::new(self.num_decks);
        }

        for _ in 0..2 {
            for player in &mut self.players {
                player.insert(self.shoe.deal().unwrap());
            }
            self.dealer.insert(self.shoe.deal().unwrap());
        }
    }

    /// Dealer peak for blackjack
    pub fn peek(&self) -> bool {
        self.dealer.is_blackjack()
    }

    /// Get the outcome for a player's hand
    pub fn get_outcome(&self, player: usize) -> Outcome {
        if self.peek() {
            return if self.players[player].is_blackjack() {
                Outcome::Push
            } else {
                Outcome::Lose
            };
        }

        if self.players[player].is_blackjack() {
            return Outcome::Blackjack;
        }

        if self.players[player].is_bust() {
            return Outcome::Lose;
        }

        if self.dealer.is_bust() {
            return Outcome::Win;
        }

        match (
            self.players[player].value().unwrap(),
            self.dealer.value().unwrap(),
        ) {
            (p, d) if p > d => Outcome::Win,
            (p, d) if p < d => Outcome::Lose,
            _ => Outcome::Push,
        }
    }

    /// A player hits
    pub fn player_hit(&mut self, player: usize) {
        if self.players[player].is_bust() {
            return;
        }

        self.players[player].insert(self.shoe.deal().unwrap());
    }

    /// The dealer hits
    pub fn dealer_hit(&mut self) {
        if self.dealer.is_bust() {
            return;
        }

        self.dealer.insert(self.shoe.deal().unwrap());
    }

    /// The dealer value
    pub fn dealer_value(&self) -> Option<u8> {
        self.dealer.value()
    }

    /// The player has busted
    pub fn player_bust(&self, player: usize) -> bool {
        self.players[player].is_bust()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n\n")?;
        writeln!(f, "Dealer: {}", self.dealer)?;
        for (i, player) in self.players.iter().enumerate() {
            writeln!(f, "Player {}: {}", i + 1, player)?;
        }
        Ok(())
    }
}
