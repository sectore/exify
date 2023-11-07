use yew::prelude::*;

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn BrokenImage(props: &Props) -> Html {
    let Props { class } = props;

    html! {
      <svg
        class={classes!(class.clone())}
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        width="32" height="32"
        viewBox="0 0 24 24">
          <path fill="currentColor" d="m21 18.15l-2-2V5H7.85l-2-2H21v15.15Zm-1.2 4.45L18.2 21H3V5.8L1.4 4.2l1.4-1.4l18.4 18.4l-1.4 1.4ZM6 17l3-4l2.25 3l.825-1.1L5 7.825V19h11.175l-2-2H6Zm7.425-6.425ZM10.6 13.4Z"/>
        </svg>
    }
}
