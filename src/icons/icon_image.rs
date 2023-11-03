use yew::prelude::*;

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IconImage)]
pub fn icon_image(props: &Props) -> Html {
    let Props { class } = props;

    html! {
      <svg
        class={classes!(class.clone())}
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        width="32" height="32"
        viewBox="0 0 24 24">
        <path fill="currentColor" d="M3 21V3h11v2H5v14h14v-9h2v11H3ZM17 9V7h-2V5h2V3h2v2h2v2h-2v2h-2ZM6 17h12l-3.75-5l-3 4L9 13l-3 4Zm-1-6v8V5v6Z"/>
      </svg>
    }
}
