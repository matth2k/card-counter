use shoo::bet::{Bet, Chip};

#[test]
fn test_bet() {
    let bet = Bet::from(Chip::TwentyFive) + Bet::from(Chip::Five) + Bet::default();
    assert_eq!(usize::from(bet), 30);
}

#[test]
fn test_bet_sub() {
    let bet = Bet::from(Chip::TwentyFive) + Bet::from(Chip::Five) + Bet::default();
    let bet = bet - Bet::from(Chip::Five);
    assert_eq!(usize::from(bet), 25);
}

#[test]
#[should_panic(expected = "overflow")]
fn test_bet_sub_fail() {
    let bet = Bet::from(Chip::TwentyFive) + Bet::from(Chip::Five) + Bet::default();
    let _bet = bet - Bet::from(Chip::One);
}
