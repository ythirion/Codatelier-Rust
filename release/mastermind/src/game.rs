use rand::seq::SliceRandom;

/// The available ball colors.
/// g -> (g)reen
/// r -> (r)ed
/// b -> (b)lue
/// y -> (y)ellow
/// k -> blac(k)
/// w -> (w)hite
const AVAILABLE_BALLS: [char; 6] = ['g', 'r', 'b', 'y', 'k', 'w'];

/// Defines the different flags.
///
/// # Variants
///
/// - `RightPosition` - The ball is at the right place in the code.
/// - `MisPlaced` - The ball is .
/// - `Invalid` - Describe this variant.
#[derive(PartialEq, Clone)]
pub enum Flag {
    RightPosition,
    MisPlaced,
    Invalid,
}

/// Represents a game instance.
///
/// # Fields
///
/// - `code` (`[char; 4]`) - The code to guess.
/// - `attempts` (`Vec<CodeAttempt>`) - The different attempts to guess the code.
/// - `is_game_over` (`bool`) - Indicates if the game is over.
/// - `is_game_active` (`bool`) - Indicates if a game is in progress.
#[derive(PartialEq, Clone)]
pub struct Game {
    pub code: [char; 4],
    pub attempts: Vec<CodeAttempt>,
    pub is_game_over: bool,
    pub is_game_active: bool,
}

/// Represents a guess.
///
/// # Fields
///
/// - `attempt` (`[char; 4]`) - The guessed code.
/// - `result` (`[Flag; 4]`) - The guess's results.
#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    pub attempt: [char; 4],
    pub result: [Flag; 4],
}

/// Generates a random code.
///
/// # Returns
///
/// - `[char; 4]` - A random code.
fn create_random_code() -> [char; 4] {
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

/// Checks if the given char is a valid ball color.
///
/// # Arguments
///
/// - `c` (`char`) - The char to check.
///
/// # Returns
///
/// - `bool` - Returns true if the given char is a valid ball color.
pub fn is_valid_char(c: char) -> bool {
    AVAILABLE_BALLS.contains(&c)
}

impl Game {
    /// Creates a new game instance with a random code.
    ///
    /// # Arguments
    ///
    /// - `is_game_active` (`bool`) - Indicates if the game is in progress.
    ///
    /// # Returns
    ///
    /// - `Self` - Returns a new game instance.
    pub fn new(is_game_active: bool) -> Self {
        Self {
            code: create_random_code(),
            attempts: vec![],
            is_game_over: false,
            is_game_active,
        }
    }

    /// Processes the given code attempt. Checks if the code attempt matches the code and updates the game.
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`) - The game instance.
    /// - `attempt` (`[char; 4]`) - The code attempt.
    ///
    /// # Returns
    ///
    /// - `Self` - Returns the modified game instance.
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
    /// Checks if the flag instance is a `Flag::RightPosition` variant.
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`) - The flag instance.
    ///
    /// # Returns
    ///
    /// - `bool` - Returns true if the flag instance is a `Flag::RightPosition` variant.
    pub fn is_right_position(&self) -> bool {
        matches!(self, Flag::RightPosition)
    }
}

impl CodeAttempt {
    /// Creates a new CodeAttempt instance.
    ///
    /// # Arguments
    ///
    /// - `attempt` (`[char; 4]`) - The guessed code.
    /// - `result` (`[Flag; 4]`) - The guess result.
    ///
    /// # Returns
    ///
    /// - `Self` - a new CodeAttempt instance.
    pub fn new(attempt: [char; 4], result: [Flag; 4]) -> Self {
        Self { attempt, result }
    }

    /// Checks if the code attempt instance matches the code.
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`) - The code attempt instance.
    ///
    /// # Returns
    ///
    /// - `bool` - Returns true if the code attempt instance matches the code.
    pub fn is_game_over(&self) -> bool {
        self.result.iter().all(|flag| flag.is_right_position())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_valid_char() {
        for c in AVAILABLE_BALLS {
            assert!(is_valid_char(c));
        }
        assert!(!is_valid_char('i'));
    }

    #[test]
    pub fn test_flag() {
        assert!(Flag::RightPosition.is_right_position());
        assert!(!Flag::MisPlaced.is_right_position());
        assert!(!Flag::Invalid.is_right_position());
    }

    #[test]
    pub fn test_code_attempt() {
        assert!(
            CodeAttempt::new(
                ['b', 'b', 'b', 'b'],
                [
                    Flag::RightPosition,
                    Flag::RightPosition,
                    Flag::RightPosition,
                    Flag::RightPosition
                ]
            )
            .is_game_over()
        );

        assert!(
            !CodeAttempt::new(
                ['b', 'b', 'b', 'b'],
                [
                    Flag::MisPlaced,
                    Flag::RightPosition,
                    Flag::RightPosition,
                    Flag::RightPosition
                ]
            )
            .is_game_over()
        );
    }
}
