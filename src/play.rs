use std::convert::TryFrom;
use std::cmp::{PartialOrd, Ordering};

use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Play {
    Rock, Paper, Scissors
}

impl Play {
    pub fn new_random(thread_rng: &mut ThreadRng) -> Self {
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
            _ => Err("You did not type rock, paper, or scissors")
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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
