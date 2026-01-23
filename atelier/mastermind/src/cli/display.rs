use crate::game::{CodeAttempt, Flag};

/// Display a color character as an emoji
pub fn color_char(c: char) -> &'static str {
    match c {
        'g' => "🟢",
        'r' => "🔴",
        'b' => "🔵",
        'y' => "🟡",
        'k' => "⚫",
        'w' => "⚪",
        _ => "❓",
    }
}

/// Display a validation flag as an emoji
pub fn flag_char(flag: &Flag) -> &'static str {
    match flag {
        Flag::RightPosition => "✅", // Right color, right position
        Flag::MisPlaced => "🟧",     // Right color, wrong position
        Flag::Invalid => "❌",       // Wrong color
    }
}

/// Display the welcome message and game rules
pub fn display_welcome() {
    println!("🎮 Welcome to Mastermind!");
    println!("==========================================");
    println!("Available colors: (g)reen, (r)ed, (b)lue, (y)ellow, blac(k), (w)hite");
    println!("Enter 4 letters to make a guess (e.g., 'grbk')");
    println!();
}

/// Display the secret code in debug mode
pub fn display_debug_code(code: &[char; 4]) {
    println!("🔍 DEBUG MODE - Secret code: {:?}\n", code);
}

/// Display the current turn number
pub fn display_turn(turn_number: usize) {
    println!("Turn {}", turn_number);
    println!("------------------------------------------");
}

/// Display all previous attempts with their results
pub fn display_attempts(attempts: &[CodeAttempt]) {
    if attempts.is_empty() {
        return;
    }

    println!("\nPrevious attempts:");
    for (i, attempt) in attempts.iter().enumerate() {
        print!("  {}. ", i + 1);
        for c in &attempt.attempt {
            print!("{} ", color_char(*c));
        }
        print!(" → ");
        for flag in &attempt.result {
            print!("{} ", flag_char(flag));
        }
        println!();
    }
    println!();
}

/// Display the victory message
pub fn display_victory(attempts_count: usize, code: &[char; 4]) {
    println!("\n🎉 Congratulations! You found the code in {} attempts!", attempts_count);
    print!("Secret code was: ");
    for c in code {
        print!("{} ", color_char(*c));
    }
    println!("\n");
}

/// Display an error message
pub fn display_error(message: &str) {
    println!("❌ {}\n", message);
}
