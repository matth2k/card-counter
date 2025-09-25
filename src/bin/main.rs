use std::io::Write;

use count::hand::Hand;
use count::table::{Outcome, Table};
use crossterm::{
    event::{Event, KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    let no_decks = 2;

    println!("Num decks {no_decks} \n\r\n");
    let mut table = Table::new(no_decks, 1);
    let mut last_outcome = Some(Outcome::Push);
    table.deal();
    loop {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Right => {
                    match last_outcome {
                        Some(_) => {
                            table.clear_hands();
                            table.deal();
                            if table.peek() {
                                print!("\rDealer has blackjack!            ");
                                std::io::stdout().flush()?;
                                last_outcome = Some(table.get_outcome(0));
                                continue;
                            }

                            if table.get_outcome(0) == Outcome::Blackjack {
                                print!("\rBlackjack!            ");
                                std::io::stdout().flush()?;
                                last_outcome = Some(table.get_outcome(0));
                                continue;
                            }

                            print!("\r{table}");
                            std::io::stdout().flush()?;
                            last_outcome = None;
                        }
                        _ => {
                            // A hit
                            table.player_hit(0);

                            if table.player_bust(0) {
                                print!("\rBust!            ");
                                std::io::stdout().flush()?;
                                last_outcome = Some(table.get_outcome(0));
                                continue;
                            }
                            print!("\r{table}");
                            std::io::stdout().flush()?;
                        }
                    }
                }
                KeyCode::Down => {

                    if last_outcome.is_some() {
                        continue;
                    }

                    while table.dealer_value().is_some_and(|v| v < 17) {
                        table.dealer_hit();
                        print!("\r{table}");
                        std::io::stdout().flush()?;
                        // Sleep for one second
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }

                    let outcome = table.get_outcome(0);
                    match outcome {
                        Outcome::Blackjack | Outcome::Win => {
                            print!("\rYou win!                           ");
                        }
                        Outcome::Push => {
                            print!("\rPush!                              ");
                        }
                        Outcome::Lose => {
                            print!("\rYou lose!                          ");
                        }
                    };
                    std::io::stdout().flush()?;
                    last_outcome = Some(outcome);
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
