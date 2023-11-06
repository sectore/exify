use yew::prelude::*;

use crate::types::AppContext;
use crate::utils::img_src;
use crate::{app_ctx::Msg, types::FileDetails};

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub file_details: FileDetails,
}

#[function_component]
pub fn Details(props: &Props) -> Html {
    let Props { file_details, .. } = props;

    let ctx = use_context::<AppContext>().unwrap();

    let on_remove = {
        let ctx = ctx.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let ctx = ctx.clone();

            ctx.dispatch(Msg::RemoveExif);
        })
    };

    let on_clear = {
        let ctx = ctx.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let ctx = ctx.clone();

            ctx.dispatch(Msg::Clear);
        })
    };

    html! {
      <>
        <img
          class="max-w-[10rem] max-h-[10rem] w-auto h-auto border-[1em] border-sky-500 "
          src={img_src(&file_details)} />

          <div class="w-full flex flex-col lg:flex-row
          justify-center items-center 
          gap-6 lg:gap-12 xl:gap-20 
          my-10 lg:my-12
          ">
            <button class="btn text-xl py-4 w-full" onclick={on_remove}>{"Remove EXIF"}</button>
            <button class="btn-neutral text-xl py-4 w-full" onclick={on_clear}>{"Cancel"}</button>
          </div>

          <h2 class="text-xl md:text-2xl font-bold text-gray-400 mb-6 ">
          {
            if &file_details.exif.len() <= &0 {
              "No exif data found".to_owned()
            } else {
                format!("{:?} EXIF data found", &file_details.exif.len())
            }
          }
          </h2>

          {
            if &file_details.exif.len() > &0 {
              html!(
                <>
                  <div class="w-full flex flex-row justify-center
                  text-gray-500 bg-gray-200 
                  text-shadow-light
                  text-xs md:text-base">
                      <div class="w-1/2 md:w-1/3 px-3 py-1 border-r border-white">{"name"}</div>
                      <div class="w-1/2 md:w-2/3 px-3 py-1">{"data"}</div>
                  </div>
                  <div class="w-full overflow-y-scroll">
                  { for file_details.exif.iter().map(|(k, v)| html! {
                      <div class="w-full flex justify-center
                      text-xs md:text-base  text-gray-500 text-shadow-light
                      odd:bg-gray-100">
                        <div class="w-1/2 md:w-1/3 px-3 py-1 border-r border-gray-200">{k.to_string()}</div>
                        <div class="w-1/2 md:w-2/3 max-w-2/3 px-3 py-1 truncate">{v.to_string()}</div>
                      </div>
                    })}
                  </div>
                </>
              )
            } else { html!{} }
          }
        </>
    }
}
