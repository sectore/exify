use yew::prelude::*;

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IconPlus)]
pub fn icon_plus(props: &Props) -> Html {
    let Props { class } = props;

    html! {
      <svg
        class={classes!(class.clone())}
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        width="32" height="32"
        viewBox="0 0 24 24">
        <path fill="currentColor" d="M11 13H5v-2h6V5h2v6h6v2h-6v6h-2v-6Z"/>
      </svg>
    }
}
