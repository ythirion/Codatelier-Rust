use yew::prelude::*;

use crate::components::{
    code_attempt::CodeAttempt, game_over::GameOver, guess_code::GuessCode, start_game::StartGame,
};

#[function_component(Game)]
pub fn game() -> Html {
    let game_state = use_state(|| crate::game::Game::new(false));

    let start_game = {
        let game_state = game_state.clone();
        Callback::from(move |_| {
            game_state.set(crate::game::Game::new(true));
        })
    };

    let submit_code_guess = {
        let game_sate = game_state.clone();
        Callback::from(move |attempt| game_sate.set((*game_sate).process_code_attempt(attempt)))
    };

    let previous_attempts = (*game_state)
        .attempts
        .iter()
        .map(|attempt| {
            html! {
                <CodeAttempt code_attempt={attempt.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        if !(*game_state).is_game_active {
            <StartGame {start_game} />
        } else if (*game_state).is_game_over {
            <GameOver game={(*game_state).clone()} />
            <StartGame {start_game} />
        } else {

            <h2>{format!("Guess the right code : Turn {}", (*game_state).attempts.len() + 1) }</h2>
            <div class={"previous-attempts"}>
                {previous_attempts}
            </div>
            <GuessCode onsubmit={submit_code_guess} />
        }
    }
}
