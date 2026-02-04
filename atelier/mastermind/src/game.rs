#![allow(dead_code, unused_imports)]

use rand::seq::SliceRandom;
use std::result;

type Ball = char;

const AVAILABLE_BALLS: [Ball; 6] = ['g', 'r', 'b', 'y', 'k', 'w'];

#[derive(PartialEq, Clone)]
pub enum Flag {
    RightPosition,
    MisPlaced,
    Invalid,
}

#[derive(PartialEq, Clone)]
pub struct Game {
    pub code: [Ball; 4],
    pub attempts: Vec<CodeAttempt>,
    pub is_game_over: bool,
    pub is_game_active: bool,
}

#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    pub attempt: [Ball; 4],
    pub result: [Flag; 4],
}

fn create_random_code() -> [Ball; 4] {
    let mut available_balls = AVAILABLE_BALLS;
    let mut rng = rand::rng();

    available_balls.shuffle(&mut rng);
    [
        available_balls[0],
        available_balls[1],
        available_balls[2],
        available_balls[3],
    ]
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

    fn to_code_attempt(&self, attempt: [Ball; 4]) -> CodeAttempt {
        let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];
        for i in 0..4 {
            result[i] = match (attempt[i] == self.code[i], self.code.contains(&attempt[i])) {
                (true, _) => Flag::RightPosition,
                (false, true) => Flag::MisPlaced,
                (false, false) => Flag::Invalid,
            };
        }
        CodeAttempt::new(attempt, result)
    }

    pub fn process_code_attempt(&self, attempt: [Ball; 4]) -> Self {
        let attempt = self.to_code_attempt(attempt);
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
        matches!(self, Flag::RightPosition)
    }
}

impl CodeAttempt {
    pub fn new(attempt: [Ball; 4], result: [Flag; 4]) -> Self {
        Self { attempt, result }
    }

    pub fn is_game_over(&self) -> bool {
        self.result.iter().all(|x| x.is_right_position())
    }
}
