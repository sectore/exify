use yew::*;

mod app_ctx;
mod components;
mod icons;
mod types;
mod utils;

use app_ctx::AppProvider;
use components::layout::Layout;
use components::skeleton::Skeleton;

#[function_component]
fn App() -> Html {
    html! {
      <AppProvider>
        // <Skeleton />
        <Layout />
      </AppProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
