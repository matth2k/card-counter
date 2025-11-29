use shoo::card::{Card, Rank, Suit};
use shoo::hand::Hand;

#[test]
fn test_hand() {
    let mut hand = Hand::default();
    hand.insert(Card::new(Suit::Spade, Rank::Ace));
    hand.insert(Card::new(Suit::Heart, Rank::King));
    assert!(hand.value().is_some());
    assert!(hand.blackjack());
}
