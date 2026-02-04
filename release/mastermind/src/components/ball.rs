use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BallProps {
    pub ball: char,
}

#[function_component(Ball)]
pub fn ball(props: &BallProps) -> Html {
    let props = props.clone();

    let color = match props.ball {
        'g' => "🟢",
        'r' => "🔴",
        'b' => "🔵",
        'y' => "🟡",
        'k' => "⚫",
        'w' => "⚪",
        _ => panic!(),
    };

    html! {
        <p class={"ball"}>{color}</p>
    }
}
