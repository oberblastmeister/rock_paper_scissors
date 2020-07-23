use std::time::Duration;

pub const SLEEP_TIME: Duration = Duration::from_secs(1);
pub const RULES: &'static str = "Type your play during the prompt. Exit anytime by pressing ctrl-c";
pub const WINNING_MSG: &'static str = "You won!";
pub const LOSING_MSG: &'static str = "You lose!";
pub const TYING_MSG: &'static str = "You tied!";
pub const PLAYER_NUMBERS: u8 = 2;
