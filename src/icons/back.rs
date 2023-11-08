use yew::prelude::*;

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Back(props: &Props) -> Html {
    let Props { class } = props;

    html! {
      <svg
        class={classes!("p-1",
        "btn-neutral",
         "rounded-full",
         "ease",
         class.clone())}
        xmlns="http://www.w3.org/2000/svg"
        width="32" height="32"
        viewBox="0 0 24 24">
        <path fill="currentColor" d="m10 18l-6-6l6-6l1.4 1.45L7.85 11H20v2H7.85l3.55 3.55L10 18Z"/>
        </svg>
    }
}
