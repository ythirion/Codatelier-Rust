use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::game::is_valid_char;

#[derive(Clone, PartialEq, Properties)]
pub struct GuessBallProps {
    pub value: char,
    pub onchange: Callback<char>,
}

#[function_component(GuessBall)]
pub fn guess_ball(props: &GuessBallProps) -> Html {
    let props = props.clone();

    let onchange = Callback::from(move |event| {
        if let Some(value) = get_value_from_select_event(event) {
            if is_valid_char(value) {
                props.onchange.emit(value);
            }
        }
    });

    if !is_valid_char(props.value) {
        panic!();
    }

    html! {
        <select class={"guess-ball"} {onchange}>
            <option value="g" selected={props.value == 'g'}>{"🟢"}</option>
            <option value="r" selected={props.value == 'r'}>{"🔴"}</option>
            <option value="b" selected={props.value == 'b'}>{"🔵"}</option>
            <option value="y" selected={props.value == 'y'}>{"🟡"}</option>
            <option value="k" selected={props.value == 'k'}>{"⚫"}</option>
            <option value="w" selected={props.value == 'w'}>{"⚪"}</option>
        </select>
    }
}

fn get_value_from_select_event(e: Event) -> Option<char> {
    let event: Event = e.dyn_into().ok()?;
    let event_target = event.target()?;
    let target: HtmlSelectElement = event_target.dyn_into().ok()?;
    let str = target.value();
    if str.len() != 1 {
        None
    } else {
        Some(str.chars().collect::<Vec<char>>()[0])
    }
}
