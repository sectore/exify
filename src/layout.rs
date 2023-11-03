use yew::*;

use crate::debug::Debug;
use crate::details::Details;
use crate::types::AppContext;
use crate::upload::Upload;
use crate::utils::img_src;

#[function_component(Layout)]
pub fn layout() -> Html {
    let ctx = use_context::<AppContext>().unwrap();

    html! {
        <div class="group w-full h-screen bg-center
        bg-gradient-to-br
        from-white
        to-gray-100 
        hover:from-gray-100 
        hover:white
        overflow-y-scroll
        flex flex-col items-center
        p-2 sm:p-6
        "
        style={if let Some(Ok(file)) = &ctx.file {
          format!("background-image: url({}", img_src(&file))
        }  else {
          "".to_owned()
        }
      }
        >

          <div class="flex flex-col items-center justify-center">
            {
              if let Some(Ok(file_details)) = &ctx.file {
                let file_details = file_details.clone();
                html! { <Details file_details={file_details} class="w-10/12 p-6" /> }
              } else {
                html! { <Upload /> }
              }
            }
            <Debug />
          </div>
        </div>
    }
}
