use yew::*;

mod app_ctx;
mod debug;
mod details;
mod download;
mod icons;
mod layout;
mod skeleton;
mod types;
mod upload;
mod utils;

use app_ctx::AppProvider;
// use layout::Layout;
use skeleton::Skeleton;

#[function_component]
fn App() -> Html {
    html! {
      <AppProvider>
        <Skeleton />
        // <Layout />
      </AppProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
