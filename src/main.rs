mod settings;

use std::io::{Write, self, stdout};
use std::cmp::{PartialOrd, Ordering};
use std::thread::sleep;
use std::convert::TryFrom;

use text_io::read;

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

use colored::Colorize;

use settings::*;

#[derive(Debug, PartialEq)]
enum Play {
    Rock, Paper, Scissors
}

impl Play {
    fn new_random(thread_rng: &mut ThreadRng) -> Self {
        use Play::*;

        match thread_rng.gen_range(0, 3) {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            _ => panic!("BUG: panic used to bypass error: [rustc E0004] [E] non-exhaustive patterns: `std::i32::MIN..=-1i32` and `3i32.. 
        =std::i32::MAX` not covered                     ensure that all possible cases are being handled, possibly by adding wildcards   or more match arms")
        }
    }
}

impl TryFrom<&String> for Play {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {

        use Play::*;

        match s.to_lowercase().trim() {
            "rock" => Ok(Rock),
            "paper" => Ok(Paper),
            "scissors" => Ok(Scissors),
            _ => Err("Could not convert the string to a Play")
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Play) -> Option<Ordering> {
        use Play::*;

        match self {
            Rock => match other {
                Rock => Some(Ordering::Equal),
                Paper => Some(Ordering::Less),
                Scissors => Some(Ordering::Greater),
            },
            Paper => match other {
                Rock => Some(Ordering::Greater),
                Paper => Some(Ordering::Equal),
                Scissors => Some(Ordering::Less),
            },
            Scissors => match other {
                Rock => Some(Ordering::Less),
                Paper => Some(Ordering::Greater),
                Scissors => Some(Ordering::Equal),
            }
        }
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

/// prints rock, paper scissors and then prompt
fn rock_paper_scissors() -> io::Result<()> {
    println!("Rock");
    sleep(SLEEP_TIME);
    println!("Paper");
    sleep(SLEEP_TIME);
    println!("Scissors");
    sleep(SLEEP_TIME);
    print!("Shoot: ");
    stdout().flush()?;

    Ok(())
}

fn choose_number_input() -> io::Result<u8> {
    let mut choice: String = read!();
    let parsed_choice;
    loop {
        match choice.parse::<u8>() {
            Ok(i) => {
                parsed_choice = i;
                break
            },
            Err(e) => {
                println!("{:?}", e);
                print!("Please retype: ");
                stdout().flush()?;
                choice = read!();
            }
        }
    }

    Ok(parsed_choice)
}

fn play_again_input() -> bool {
    let again: char = read!();
    loop {
        match again {
            'y' => return true,
            'n' => return false,
            _ => println!("Please type the letter y or n"),
        }
    }
} 

/// keeps asking user for input if user input could not be converted
fn rock_paper_scissors_input() -> io::Result<Play> {
    let mut input: String = read!();
    let play;
    loop {
        match Play::try_from(&input) { 
            Ok(item) => {
                play = item;
                break;
            },
            Err(e) => {
                println!("{}", e);
                print!("Please retype: ");
                stdout().flush()?;
                input = read!();
            }
        }
    }

    Ok(play)
}

fn main() -> io::Result<()> {
    clear_screen();
    println!("{}\n", RULES);
    sleep(SLEEP_TIME * 2);
    print!("Do you want to play against a friend or a bot? (pick 1 or 2) ");
    stdout().flush()?;
    let choice = choose_number_input()?;

    let mut thread_rng = thread_rng();

    // game loop
    if choice == 1 {
        loop {
            rock_paper_scissors()?;
            let play = rock_paper_scissors_input()?;

            // let bot play
            println!("The bot is playing...");
            let bot_play = Play::new_random(&mut thread_rng);

            sleep(SLEEP_TIME);

            println!("The bot played {:?}", bot_play);

            sleep(SLEEP_TIME);

            if play > bot_play {
                println!("{}", WINNING_MSG);
            } else if play == bot_play {
                println!("{}", TYING_MSG);
            } else {
                println!("{}", LOSING_MSG);
            }

            print!("Do you want to play again? [y/n]");
            stdout().flush()?;
            let again: String = read!();
            if again == "n" {
                break
            }

            clear_screen();
        }
    } else {
        loop {
            rock_paper_scissors()?;
            let play = rock_paper_scissors_input()?;


            println!("The bot is playing...");
            let bot_play = Play::new_random(&mut thread_rng);

            sleep(SLEEP_TIME);

            println!("The bot played {:?}", bot_play);

            sleep(SLEEP_TIME);

            if play > bot_play {
                println!("{}", WINNING_MSG);
            } else {
                println!("{}", LOSING_MSG);
            }

            print!("Do you want to play again? [y/n] ");
            stdout().flush()?;
            if !play_again_input() {
                break
            }
        }
    }

    Ok(())
}
