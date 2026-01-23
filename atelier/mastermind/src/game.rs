use rand::seq::SliceRandom;

const AVAILABLE_BALLS: [char; 6] = ['g', 'r', 'b', 'y', 'k', 'w'];

#[derive(PartialEq, Clone)]
pub enum Flag {
    RightPosition,
    MisPlaced,
    Invalid,
}

#[derive(PartialEq, Clone)]
pub struct Game {
    pub code: [char; 4],
    pub attempts: Vec<CodeAttempt>,
    pub is_game_over: bool,
    pub is_game_active: bool,
}

#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    pub attempt: [char; 4],
    pub result: [Flag; 4],
}

fn create_random_code() -> [char; 4] {
    let mut available_balls = AVAILABLE_BALLS.clone();
    let mut rng = rand::rng();
    available_balls.shuffle(&mut rng);
    return [
        available_balls[0],
        available_balls[1],
        available_balls[2],
        available_balls[3],
    ];
}

pub fn is_valid_char(c: char) -> bool {
    AVAILABLE_BALLS.contains(&c)
}

impl Game {
    pub fn new(is_game_active: bool) -> Self {
        Self {
            code: create_random_code(),
            attempts: vec![],
            is_game_over: false,
            is_game_active,
        }
    }

    pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
        let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];

        for i in 0..4 {
            result[i] = if attempt[i] == self.code[i] {
                Flag::RightPosition
            } else if self.code.contains(&attempt[i]) {
                Flag::MisPlaced
            } else {
                Flag::Invalid
            }
        }

        let attempt = CodeAttempt::new(attempt, result);

        Self {
            is_game_over: attempt.is_game_over(),
            attempts: {
                let mut attempts = self.attempts.clone();
                attempts.push(attempt);
                attempts
            },
            ..*self
        }
    }
}

impl Flag {
    pub fn is_right_position(&self) -> bool {
        match self {
            Flag::RightPosition => true,
            _ => false,
        }
    }
}

impl CodeAttempt {
    pub fn new(attempt: [char; 4], result: [Flag; 4]) -> Self {
        Self { attempt, result }
    }

    pub fn is_game_over(&self) -> bool {
        self.result.iter().all(|flag| flag.is_right_position())
    }
}
