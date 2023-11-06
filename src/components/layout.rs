use yew::*;

use crate::components::add::Add;
use crate::components::details::Details;
use crate::icons::Logo;
use crate::types::{AppContext, FileDetails};
use crate::utils::img_src;

#[derive(PartialEq, Clone)]
enum ViewState {
    Add,
    Details(FileDetails),
    Error(String),
}

#[function_component]
pub fn Layout() -> Html {
    let ctx = use_context::<AppContext>().unwrap();

    let view_state = use_state(|| ViewState::Add);

    use_effect_with(ctx.file.clone(), {
        let view_state = view_state.clone();
        move |f| match f {
            Some(Ok(file)) => {
                view_state.set(ViewState::Details(file.clone()));
            }
            Some(Err(e)) => {
                view_state.set(ViewState::Error(e.to_string()));
            }
            None => {
                view_state.set(ViewState::Add);
            }
        }
    });

    html! {
        <div class="group w-full h-screen bg-center
        bg-gradient-to-br
        from-sky-900 
        to-sky-400 
        hover:from-sky-400 
        hover:to-sky-900
        flex flex-col items-center justify-center
        p-5 md:p-20
        bg-cover
        relative
        ease
        "
        style={if let ViewState::Details(f) = &*view_state {
          format!("background-image: url({}", img_src(&f))
        }  else {
          "".to_owned()
        }
      }
      >
      <div class="absolute left-0 right-0 top-0 flex justify-center">
        <div class="flex items-center justify-center py-2 px-6 sm:px-8 rounded-b-2xl bg-white bg-opacity-95 drop-shadow-md">
          <Logo class="!w-auto !h-6 sm:!h-8 text-sky-600 hover:text-sky-500 ease" />
        </div>
      </div>
        <div class="flex w-full md:w-[80%] h-full
        flex-col items-center
        drop-shadow-md 
        bg-white 
        my-20 
        p-8 md:p-12 rounded-xl md:rounded-3xl 
        overflow-hidden
        ease ">
          { match &*view_state {
            ViewState::Add => html!(<Add />),
            ViewState::Details(f) => html!(<Details file_details={f.clone()} />),
            ViewState::Error(e) => html!(<p>{e}</p>),
            }
          }

        </div>
    </div>
    }
}
