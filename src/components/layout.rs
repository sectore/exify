use yew::*;

use crate::components::add::Add;
use crate::components::details::Details;
use crate::icons::Logo;
use crate::types::{AppContext, FileError};
use crate::utils::img_src;

#[function_component]
pub fn Layout() -> Html {
    let ctx = use_context::<AppContext>().unwrap();

    html! {
        <div class="group w-full h-screen bg-center
        bg-gradient-to-br
        from-sky-900 
        to-sky-400 
        hover:from-sky-400 
        hover:to-sky-900
        flex flex-col items-center justify-center
        px-8 py-14 md:p-20
        bg-cover
        relative
        ease
        "
        style={if let Some(Ok(f)) = ctx.file.clone() {
          format!("background-image: url({}", img_src(&f))
        }  else {
          "".to_owned()
        }
      }
      >
      <div class="absolute left-0 right-0 top-0 flex justify-center">
        <div class="flex items-center justify-center py-2 px-6 md:px-8 rounded-b-2xl bg-white bg-opacity-95 drop-shadow-md">
          <Logo class="!w-auto !h-6 md:!h-8 text-sky-600 hover:text-sky-500 ease" />
        </div>
      </div>
        <div class="flex w-full lg:w-[80%] h-full
        flex-col items-center
        drop-shadow-md
        bg-white
        rounded-xl md:rounded-3xl
        p-6 md:p-20 
        overflow-hidden
        relative
        ease "
        >
        { match ctx.file.clone() {
            None => {
              html!(<Add />)
            }
            Some(Err(FileError::DragDropFailed(_))) => {
              html!(<Add />)
            }
            Some(_) => {
              html!(<Details />)
            }
          }
        }
        </div>
    </div>
    }
}
