use yew::prelude::*;

use crate::{components::guess_ball::GuessBall, game::is_valid_char};

/// The GuessCode component's properties.
///
/// # Fields
///
/// - `onsubmit` (`Callback<[char; 4]>`) - The on submit event callbak.
#[derive(Clone, PartialEq, Properties)]
pub struct GuessCodeProps {
    pub onsubmit: Callback<[char; 4]>,
}

/// The GuessCode's Html component.
///
/// # Arguments
///
/// - `props` (`&GuessCodeProps`) - The component's properties.
///
/// # Returns
///
/// - `Html` - Returns an Html component.
#[function_component(GuessCode)]
pub fn guess_code(props: &GuessCodeProps) -> Html {
    let props = props.clone();
    let state = use_state(|| ['k', 'k', 'k', 'k']);

    let selects = (*state)
        .iter()
        .enumerate()
        .map(|(index, ch)| {
            let onchange = {
                let state = state.clone();
                Callback::from(move |c| {
                    let mut updated = (*state).clone();
                    if is_valid_char(c) {
                        updated[index] = c;
                        state.set(updated);
                    }
                })
            };

            html! {
                <GuessBall value={*ch} {onchange} />
            }
        })
        .collect::<Html>();

    let onclick: Callback<MouseEvent> = Callback::from(move |_| props.onsubmit.emit(*state));

    html! {
        <div class={"guess-code"}>
            {selects}
            <button {onclick} class="button">
                <div class={"button-top"}>{"Guess"}</div>
                <div class={"button-bottom"}></div>
                <div class={"button-base"}></div>
            </button>
        </div>
    }
}
