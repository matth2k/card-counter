![](https://github.com/matth2k/shoo/actions/workflows/rust.yml/badge.svg)
[![Docs](https://img.shields.io/badge/docs-github--pages-blue)](https://matth2k.github.io/shoo/)
[![crates.io](https://img.shields.io/badge/crates.io-github--pages-blue)](https://crates.io/crates/shoo)

# Shoo: A Library for Blackjack training

## Description

A Rust library to assist in creating your own blackjack trainer or game.

## Getting Started

Below is a minimal example to get you started:

```rust
use shoo::{Card, Hand, Rank, Suit};

#[test]
fn test_hand() {
    let mut hand = Hand::default();
    hand.insert(Card::new(Suit::Spade, Rank::Ace));
    hand.insert(Card::new(Suit::Heart, Rank::King));
    assert!(hand.value().is_some());
    assert!(hand.blackjack());
}

```
