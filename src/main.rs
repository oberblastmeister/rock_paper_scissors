mod settings;

use std::io::{Write, self, stdout};
use std::cmp::{PartialOrd, Ordering};
use std::thread::sleep;
use std::convert::TryFrom;

use text_io::read;


use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

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
fn main() -> io::Result<()> {
    clear_screen();
    // print!("Do you want to play against a friend or play against a bot? (pick 1 or 2)");
    // stdout().flush()?;
    // let choice: i8 = read!();
    let mut thread_rng = thread_rng();

    loop {
        println!("Rock");
        sleep(SLEEP_TIME);
        println!("Paper");
        sleep(SLEEP_TIME);
        println!("Scissors");
        sleep(SLEEP_TIME);
        print!("Shoot!: ");
        stdout().flush()?;

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

        print!("Do you want to play again? [y/n]");
        stdout().flush()?;
        let again: String = read!();
        if again == "n" {
            break
        }

        clear_screen();
    }

    Ok(())
}
