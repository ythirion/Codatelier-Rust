use clap::Parser;
use mastermind::cli::runner;

#[derive(Parser)]
#[command(name = "Mastermind CLI")]
#[command(about = "Play Mastermind in your terminal", long_about = None)]
struct Cli {
    /// Enable debug mode (shows the secret code)
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();
    runner::run_game(cli.debug);
}
