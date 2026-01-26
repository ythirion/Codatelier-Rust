use yew::prelude::*;

use crate::components::ball::Ball;

/// The GameOver component's properties.
///
/// # Fields
///
/// - `game` (`crate`) - The game instance.
#[derive(Clone, PartialEq, Properties)]
pub struct GameOverProps {
    pub game: crate::game::Game,
}

/// The GameOver Html component.
///
/// # Arguments
///
/// - `props` (`&GameOverProps`) - The component's properties.
///
/// # Returns
///
/// - `Html` - Returns an Html component.
#[function_component(GameOver)]
pub fn game_over(props: &GameOverProps) -> Html {
    let props = props.clone();

    let display_code = match props.game.attempts.last() {
        None => html! {},
        Some(attempt) => {
            let code = attempt
                .attempt
                .iter()
                .map(|ch| html! { <Ball ball={ch} /> })
                .collect::<Html>();
            html! {
                <>
                    <div class={"solution"}>
                        {code}
                    </div>
                </>
            }
        }
    };

    html! {
        <>
            <h2>{ format!("You have guessed the right code in {} turns !", props.game.attempts.len()) }</h2>
            {display_code}
        </>
    }
}
