use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use yew::*;

use crate::types::AppContext;

#[function_component]
pub fn Debug() -> Html {
    let ctx = use_context::<AppContext>().unwrap();

    html! {
      match ctx.file.clone() {
        Some(result) => match result {
          Ok(file) => html! { <div>
            <h1>{file.name}</h1>
            <h2>{"Exif data lenght "} {file.exif.len()}</h2>
            <img class="w-1/2 h-auto" src={format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data))} />
          </div> },
          Err(err) => html! { <div>{err.to_string()}</div> },
        },
        None => html! { <div>{"no file selected"}</div>}
      }
    }
}
