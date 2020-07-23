mod settings;
mod play;

use std::io::{Write, self, stdout, stdin};
use std::thread::sleep;
use std::convert::TryFrom;

use text_io::read;

use rand::thread_rng;
use rand::rngs::ThreadRng;

use colored::Colorize;

use settings::*;
use play::Play;

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
                match i {
                    1 | 2 => {
                        parsed_choice = i;
                        break
                    }
                    _ => {
                        println!("You choice must be 1 or 2.");
                        print!("Please retype: ");
                        stdout().flush()?;
                        choice = read!();
                    }
                }
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

fn confirm(msg: &str) -> io::Result<()> {
    print!("{}", msg);
    stdout().flush()?;
    let mut _s = String::new();
    stdin().read_line(&mut _s)?;

    clear_screen();

    sleep(SLEEP_TIME);

    Ok(())
}

fn play_bot(thread_rng: &mut ThreadRng) -> io::Result<()> {
    loop {
        rock_paper_scissors()?;
        let play = rock_paper_scissors_input()?;

        // let bot play
        println!("The bot is playing...");
        let bot_play = Play::new_random(thread_rng);

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

    Ok(())
}

fn play_friend() -> io::Result<()> {
    loop {
        let mut plays = Vec::with_capacity(PLAYER_NUMBERS as usize);
        for i in 1..=PLAYER_NUMBERS {
            let message = format!("Is player {} ready? (press enter to continue) ", i);
            confirm(&message)?;
            rock_paper_scissors()?;
            let play = rock_paper_scissors_input()?;
            plays.push(play);
            clear_screen();
        }

        confirm("Are you ready to see the results? (press enter to continue) ")?;

        for i  in 1..=PLAYER_NUMBERS {
            println!("Player {} played {:?}", i, plays[i as usize - 1]);
        }

        if plays[0] > plays[1] {
            println!("Player 1 has won!");
        } else if plays[0] == plays[1] {
            println!("Player 1 and Player 2 tied!");
        } else {
            println!("Player 2 has won!");
        }

        print!("Do you want to play again? [y/n] ");
        stdout().flush()?;
        if !play_again_input() {
            break
        }
    }

    Ok(())
}

fn prelude() -> io::Result<u8> {
    println!("{}\n", RULES);
    sleep(SLEEP_TIME);
    print!("Do you want to play against a bot or a friend? (pick 1 or 2) ");
    stdout().flush()?;

    Ok(choose_number_input()?)
}

fn main() -> io::Result<()> {
    clear_screen();
    let choice = prelude()?;
    clear_screen();

    let mut thread_rng = thread_rng();

    if choice == 1 {
        play_bot(&mut thread_rng)?;
    } else {
        play_friend()?;
    }

    Ok(())
}
