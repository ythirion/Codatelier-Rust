use yew::prelude::*;

use crate::components::game::Game;

#[component]
pub fn App() -> Html {
    html! {
        <section class={"mastermind"}>
            <h1 class={"title"}>{ "Mastermind" }</h1>
            <Game />
        </section>
    }
}
