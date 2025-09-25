use std::io::Write;

use count::hand::Hand;
use count::shoe::Shoe;
use crossterm::{
    event::{Event, KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    let no_decks = 2;

    println!("Num decks {no_decks} \n\r\n");
    let mut shoe = Shoe::new(no_decks);
    let mut hand = Hand::default();
    while !shoe.is_empty() {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Right => {
                    if hand.is_bust() {
                        print!(
                            "\r                                                                      "
                        );
                        std::io::stdout().flush()?;
                        hand = Hand::default();
                    } else {
                        let count = shoe.running_count();
                        let card = shoe.deal();
                        match card {
                            Some(card) => {
                                hand.insert(card);
                                print!(
                                    "\r                                                              "
                                );
                                std::io::stdout().flush()?;
                                print!("\r\t{}\tCount: {:.1}\t", hand, count);
                                std::io::stdout().flush()?;
                            }
                            None => break,
                        }
                    }
                }
                KeyCode::Char('c')
                    if key_event
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                {
                    break;
                }
                KeyCode::Esc => {
                    break;
                }
                _ => (),
            }
        }
    }

    disable_raw_mode()
}
