#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "rock" => Hand::Rock,
            "paper" => Hand::Paper,
            "scissors" => Hand::Scissors,
            _ => panic!("unrecognized command"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => Some(std::cmp::Ordering::Greater),
            _ if self == other => Some(std::cmp::Ordering::Equal),
            _ => Some(std::cmp::Ordering::Less),
        }
    }
}

fn rps(p1: &str, p2: &str) -> &'static str {
    let p1: Hand = p1.into();
    let p2: Hand = p2.into();
    if p1 > p2 {
        "Player 1 won!"
    } else if p1 < p2 {
        "Player 2 won!"
    } else {
        "Draw!"
    }
}
