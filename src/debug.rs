use yew::*;

use crate::types::AppContext;

#[function_component]
pub fn Debug() -> Html {
    let ctx = use_context::<AppContext>().unwrap();

    html! {
      <div class="bg-white"> {

        match ctx.file.clone() {
          Some(result) => match result {
            Ok(file) => html! { <div>
              <h1>{file.name}</h1>
              <h2>{"Exif data lenght "} {file.exif.len()}</h2>
              </div> },
              Err(err) => html! { <div>{err.to_string()}</div> },
            },
            None => html! { <div>{"no file selected"}</div>}
          }
        }
      </div>
    }
}
