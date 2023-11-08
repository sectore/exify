use yew::prelude::*;

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Close(props: &Props) -> Html {
    let Props { class } = props;

    html! {
      <svg
        class={classes!("p-2",
        "btn-neutral",
         "rounded-full",
         "ease",
         class.clone())}
        xmlns="http://www.w3.org/2000/svg"
        width="32" height="32"
        viewBox="0 0 24 24">
        <path fill="currentColor" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41z"/>
        </svg>
    }
}
