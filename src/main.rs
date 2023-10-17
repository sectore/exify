use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
      <h1 class="text-2xl font-medium text-sky-500">
        {"exify"}
      </h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
