use yew::*;

use crate::icons::{IconImage, IconPlus, Logo, Spinner};
use crate::types::AppContext;

#[function_component(Skeleton)]
pub fn skeleton() -> Html {
    let _ctx = use_context::<AppContext>().unwrap();

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
        >
          <div class="flex justify-center my-4 sm:my-6 ">
            <Logo class="w-auto h-8 sm:h-10 text-gray-200 " />
            </div>
          <div class="w-full flex flex-grow items-center justify-center">
            <div class="flex flex-col items-center bg-white border border-gray-300 bg-opacity-70 group-hover:bg-opacity-90 p-16 sm:p-20 rounded-xl sm:rounded-3xl ease">
              <IconImage class="w-36 h-36 sm:w-56 sm:h-56 mb-2 sm:mb-4 text-gray-200 group-hover:text-blue-300 group-hover:scale-105 text-shadow-light ease" />
              <p class="text-gray-300 font-bold text-center text-2xl sm:text-4xl uppercase text-shadow-light group-hover:text-blue-300 group-hover:scale-105 ease ">{"Drop your image here"}</p>
              <p class="text-gray-300 font-bold text-center text-xl sm:text-2xl uppercase text-shadow-light mt-2 sm:mt-6 mb-4 sm:mb-8">{"or"}</p>
              <button class="
                justify-center
                flex items-center
                rounded-xl button
                text-opacity-90
                hover:text-opacity-100
                hover:scale-105
                font-bold 
                text-xl sm:text-4xl
                w-full sm:w-auto
                py-2 sm:py-4 pl-4 sm:pl-10 pr-2 sm:pr-4 mb-3 
                uppercase 
                shadow-lg
                ease
              ">
                {"Select image"}
                <IconPlus class="w-8 h-8 sm:w-12 sm:h-12 ml-2 sn:ml-4" />
              </button>
              <p class="text-gray-300 text-sm sm:text-base text-shadow-light">{"Supports jpg, png, webp"}</p>
              <Spinner class="w-10 h-10 text-blue-300 mt-10" />
            </div>
          </div>
        </div>
    }
}
