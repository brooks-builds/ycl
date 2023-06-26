pub use yew::prelude::*;
pub use zxcvbn::zxcvbn;

use crate::elements::progress_bar::BBProgressBar;

const PERCENTAGES: [&str; 5] = ["0", "25", "50", "75", "100"];

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub password: AttrValue,
}

#[function_component(BBPasswordStrength)]
pub fn component(props: &Props) -> Html {
    let score = if let Ok(strength) = zxcvbn(props.password.as_str(), &[]) {
        strength.score()
    } else {
        0
    };

    let percentage = PERCENTAGES[score as usize];

    html! {
        <BBProgressBar label="Password Strength" {percentage} />
    }
}
