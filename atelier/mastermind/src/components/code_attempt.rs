use yew::prelude::*;

use crate::{components::ball::Ball, components::flag::Flag};

#[derive(Clone, PartialEq, Properties)]
pub struct CodeAttemptProps {
    pub code_attempt: crate::game::CodeAttempt,
}

#[function_component(CodeAttempt)]
pub fn code_attempt(props: &CodeAttemptProps) -> Html {
    let props = props.clone();

    let guess = props
        .code_attempt
        .attempt
        .iter()
        .map(|b| {
            html! { <Ball ball={*b} />}
        })
        .collect::<Html>();

    let answer = props
        .code_attempt
        .result
        .into_iter()
        .map(|f| html! { <Flag flag={f} /> })
        .collect::<Html>();

    html! {
        <div class={"code-attempt"}>
            <div class={"guess"}>
                {guess}
                <div class={"answer"}>
                    {answer}
                </div>
            </div>
        </div>
    }
}
