use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
      <h1>
        {"exify"}
      </h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
