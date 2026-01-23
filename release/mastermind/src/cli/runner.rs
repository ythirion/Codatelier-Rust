use crate::game::Game;
use std::io::{self, Write};

use super::display;

/// Run the main game loop
pub fn run_game(debug_mode: bool) {
    display::display_welcome();

    let mut game = Game::new(true);

    if debug_mode {
        display::display_debug_code(&game.code);
    }

    loop {
        display::display_turn(game.attempts.len() + 1);
        display::display_attempts(&game.attempts);

        match get_user_input() {
            Ok(attempt) => {
                game = game.process_code_attempt(attempt);

                if game.is_game_over {
                    display::display_victory(game.attempts.len(), &game.code);
                    break;
                }
            }
            Err(err) => {
                display::display_error(&err);
            }
        }

        println!();
    }
}

/// Get and validate user input for a code attempt
fn get_user_input() -> Result<[char; 4], String> {
    print!("Your guess: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    validate_input(input.trim())
}

/// Validate user input and convert to attempt array
fn validate_input(input: &str) -> Result<[char; 4], String> {
    let input = input.to_lowercase();

    if input.len() != 4 {
        return Err("Please enter exactly 4 letters!".to_string());
    }

    let chars: Vec<char> = input.chars().collect();

    if !chars.iter().all(|&c| crate::game::is_valid_char(c)) {
        return Err("Invalid color! Use only: g, r, b, y, k, w".to_string());
    }

    Ok([chars[0], chars[1], chars[2], chars[3]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_valid() {
        assert!(validate_input("grbk").is_ok());
        assert!(validate_input("GRBK").is_ok());
        assert!(validate_input("wyrg").is_ok());
    }

    #[test]
    fn test_validate_input_wrong_length() {
        assert!(validate_input("grb").is_err());
        assert!(validate_input("grbky").is_err());
        assert!(validate_input("").is_err());
    }

    #[test]
    fn test_validate_input_invalid_chars() {
        assert!(validate_input("grba").is_err());
        assert!(validate_input("1234").is_err());
        assert!(validate_input("grb!").is_err());
    }
}
