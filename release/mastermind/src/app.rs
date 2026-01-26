use yew::prelude::*;

use crate::components::game::Game;

/// The root component of the yew application.
///
/// # Returns
///
/// - `Html` - An Html component.
#[component]
pub fn App() -> Html {
    html! {
        <section class={"mastermind"}>
            <h1 class={"title"}>{ "Mastermind" }</h1>
            <Game />
        </section>
    }
}
