use yew::prelude::*;

/// The Ball component's properties.
///
/// # Fields
///
/// - `ball` (`char`) - The ball color.
#[derive(Clone, PartialEq, Properties)]
pub struct BallProps {
    pub ball: char,
}

/// The Ball Html component.
///
/// # Arguments
///
/// - `props` (`&BallProps`) - The component's properties.
///
/// # Returns
///
/// - `Html` - Returns an Html component.
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
