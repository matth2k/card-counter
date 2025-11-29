/*!

  A blackjack table

*/

use crate::{hand::Hand, shoe::Shoe};
use std::fmt::Display;

/// Represents the outcome of a hand
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
    player_hands: Vec<Hand>,
    /// The shoe
    shoe: Shoe,
    /// The max deck penetration before reshuffle
    max_penetration: f32,
    /// The number of decks to play with
    num_decks: usize,
    /// Cards are currently dealth
    dealt: bool,
}

impl Table {
    /// Creates a new blackjack table with `num_decks` decks and `num_spots` bettings spots and `max_penetration` (from 0.0-1.0 before shuffling)
    pub fn new(num_decks: usize, num_spots: usize, max_penetration: f32) -> Self {
        let shoe = Shoe::new(num_decks);
        let player_hands = vec![Hand::default(); num_spots];
        let dealer = Hand::default();

        let max_penetration = max_penetration.clamp(0.0, 1.0);

        Self {
            dealer,
            player_hands,
            shoe,
            max_penetration,
            num_decks,
            dealt: false,
        }
    }

    /// Resets the table
    ///
    /// # Panics
    /// Panics if there are cards currently dealt
    pub fn reset(&mut self) {
        if self.dealt {
            panic!("Cannot reset table while cards are dealt");
        }

        self.shoe = Shoe::new(self.num_decks);
        self.dealer = Hand::default();
        for player in &mut self.player_hands {
            *player = Hand::default();
        }
    }

    /// Clear the current hands no matter the state of the table
    pub fn clear_hands(&mut self) {
        self.dealer = Hand::default();
        for player in &mut self.player_hands {
            *player = Hand::default();
        }
        self.dealt = false;
    }

    /// Deal the initial hands for player and dealers
    ///
    /// # Panics
    /// Panics if there are already cards dealt on the table
    pub fn deal(&mut self) {
        if self.dealt {
            panic!("Cannot deal when hands are currently on the table");
        }

        for player in &self.player_hands {
            debug_assert!(player.is_empty());
        }

        if self.shoe.penetration() > self.max_penetration {
            self.shoe = Shoe::new(self.num_decks);
        }

        self.dealt = true;

        for _ in 0..2 {
            for player in &mut self.player_hands {
                player.insert(self.shoe.deal().unwrap());
            }
            self.dealer.insert(self.shoe.deal().unwrap());
        }
    }

    /// Returns true if the dealer has blackjack
    pub fn peek(&self) -> bool {
        self.dealer.blackjack()
    }

    /// Returns a reference to the hand of player `player`
    pub fn player_hand(&self, player: usize) -> &Hand {
        &self.player_hands[player]
    }

    /// Get the outcome for a player's hand
    pub fn get_outcome(&self, player: usize) -> Outcome {
        if self.peek() {
            return if self.player_hand(player).blackjack() {
                Outcome::Push
            } else {
                Outcome::Lose
            };
        }

        if self.player_hand(player).blackjack() {
            return Outcome::Blackjack;
        }

        if self.player_hand(player).busted() {
            return Outcome::Lose;
        }

        if self.dealer.busted() {
            return Outcome::Win;
        }

        match (
            self.player_hand(player).value().unwrap(),
            self.dealer.value().unwrap(),
        ) {
            (p, d) if p > d => Outcome::Win,
            (p, d) if p < d => Outcome::Lose,
            _ => Outcome::Push,
        }
    }

    /// Deal a player an additional card. Returns true if the player busted.
    ///
    /// # Panics
    /// Panics if there are no cards currently dealt
    pub fn player_hit(&mut self, player: usize) -> bool {
        if !self.dealt {
            panic!("Cannot hit when no cards are dealt");
        }

        if self.player_hand(player).busted() {
            return true;
        }

        self.player_hands[player].insert(self.shoe.deal().unwrap());

        self.player_hand(player).busted()
    }

    /// The dealer hits. Returns true if the dealer busted.
    pub fn dealer_hit(&mut self) -> bool {
        if !self.dealt {
            panic!("Cannot hit when no cards are dealt");
        }

        if self.dealer.busted() {
            return true;
        }

        self.dealer.insert(self.shoe.deal().unwrap());

        self.dealer.busted()
    }

    /// The dealer value
    pub fn dealer_value(&self) -> Option<u8> {
        if !self.dealt {
            return None;
        }

        self.dealer.value()
    }

    /// An iterator over the player hands
    pub fn player_hands(&self) -> impl Iterator<Item = &Hand> {
        self.player_hands.iter()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n\n")?;
        writeln!(f, "Dealer: {}", self.dealer)?;
        for (i, player) in self.player_hands().enumerate() {
            writeln!(f, "Player {}: {}", i + 1, player)?;
        }
        Ok(())
    }
}
