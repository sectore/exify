use yew::*;

mod app_ctx;
mod debug;
mod types;
mod upload;
mod utils;

use app_ctx::AppProvider;
use debug::Debug;
use upload::Upload;

#[function_component]
fn App() -> Html {
    html! {
      <AppProvider>
        <div class="w-full h-screen">
        <header class="text-center bg py-2">
        <h1 class="px-6 py-1 bg-white rounded-full text-base font-bold text-sky-500 inline">{"exify!"}</h1>
        </header>
        <div class="flex flex-col items-center justify-center h-full">
          <Upload />
          <Debug />
        </div>
        </div>
      </AppProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
