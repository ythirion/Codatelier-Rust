use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StartGameProps {
    pub start_game: Callback<()>,
}

#[function_component(StartGame)]
pub fn start_game(props: &StartGameProps) -> Html {
    let props = props.clone();

    let onclick = Callback::from(move |_| props.start_game.emit(()));
    html! {
        <div class={"start-game"}>
            <button {onclick} class="button large-button">
                <div class={"button-top"}>{"Start game"}</div>
                <div class={"button-bottom"}></div>
                <div class={"button-base"}></div>
            </button>
        </div>
    }
}
