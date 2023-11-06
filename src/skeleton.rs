use exif::Tag;
use yew::*;

use crate::icons::{Image, Logo, Plus, Spinner};
use crate::types::{AppContext, ExifMap};

#[derive(PartialEq)]
enum ViewState {
    Add,
    Exif,
}

#[function_component(Skeleton)]
pub fn skeleton() -> Html {
    let _ctx = use_context::<AppContext>().unwrap();

    let view_state = use_state(|| ViewState::Exif);

    let exif_array = [
        (Tag::ImageDescription, "A beautiful landscape"),
        (Tag::Make, "Canon"),
        (Tag::Model, "EOS 80D"),
        (Tag::Orientation, "Horizontal (normal)"),
        (Tag::XResolution, "300"),
        (Tag::YResolution, "300"),
        (Tag::ResolutionUnit, "Inch"),
        (Tag::Software, "Adobe Photoshop CS6 (Windows)"),
        (Tag::DateTime, "2023:04:12 15:05:24"),
        (Tag::Artist, "John Doe"),
        (Tag::YCbCrPositioning, "Co-sited"),
        (Tag::ExposureTime, "1/320"),
        (Tag::FNumber, "8.0"),
        (Tag::ExposureProgram, "Manual"),
        (Tag::ISOSpeed, "100"),
        (Tag::ExifVersion, "0231"),
        (Tag::DateTimeOriginal, "2023:04:12 15:05:24"),
        (Tag::DateTimeDigitized, "2023:04:12 15:05:24"),
        (Tag::ShutterSpeedValue, "1/320"),
        (Tag::ApertureValue, "6.375"),
    ];

    let exif_data: ExifMap = exif_array
        .iter()
        .map(|&(k, v)| (k, v.to_string()))
        .collect();

    let render_add = {
        html!(
          <>
            <Image class="w-36 h-36 sm:w-56 sm:h-56 mb-2 sm:mb-4 text-sky-600 group-hover:text-sky-500 group-hover:scale-105 text-shadow-light ease" />
            <p class="text-sky-600 font-bold text-center text-2xl sm:text-4xl uppercase text-shadow-light group-hover:text-sky-500 group-hover:scale-105 ease ">{"Drop image here"}</p>
            <p class="text-gray-300 font-bold text-center text-xl sm:text-2xl uppercase text-shadow-light mt-2 sm:mt-6 mb-4 sm:mb-8">{"or"}</p>
            <button class="btn
              pl-4 sm:pl-10 pr-2 sm:pr-4 mb-3
              w-full sm:w-auto
              ">
              {"Select image"}
              <Plus class="w-8 h-8 sm:w-12 sm:h-12 ml-2 sn:ml-4" />
            </button>
            <p class="text-gray-300 text-sm sm:text-base text-shadow-light">{"Supports jpg, png, webp"}</p>
            // <Spinner class="w-10 h-10 text-sky-300 mt-10" />
        </>)
    };

    let render_exif = {
        html! {
        <>
          <img
            class="max-w-[10rem] max-h-[10rem] w-auto h-auto border-[1em] border-sky-500 "
            src="/assets/demo.jpg" />
            <div class="w-full flex flex-col lg:flex-row
            justify-center items-center 
            gap-6 lg:gap-12 xl:gap-20 
            my-10 lg:my-12
            ">
              <button class="btn text-xl py-4 w-full ">{"Remove EXIF"}</button>
              <button class="btn-neutral text-xl py-4 w-full">{"Cancel"}</button>
            </div>

          // <h2 class="text-md md:text-lg text-red-500 mb-3 ">{"Error while parsing EXIF data"}</h2>
          <h2 class="text-xl md:text-2xl font-bold text-gray-400 mb-6 ">{"20 EXIF data found"}</h2>

          <div class="w-full flex flex-row justify-center
            text-gray-500 bg-gray-200 
            text-shadow-light
            text-xs md:text-base">
                <div class="w-1/2 md:w-1/3 px-3 py-1 border-r border-white">{"name"}</div>
                <div class="w-1/2 md:w-2/3 px-3 py-1">{"data"}</div>
          </div>
          <div class="w-full overflow-y-scroll">
            { for exif_data.iter().map(|(k, v)| html! {
              <div class="w-full flex justify-center
              text-xs md:text-base  text-gray-500 text-shadow-light
              odd:bg-gray-100">
                <div class="w-1/2 md:w-1/3 px-3 py-1 border-r border-gray-200">{k.to_string()}</div>
                <div class="w-1/2 md:w-2/3 px-3 py-1">{v.to_string()}</div>
              </div>
            })}
          </div>
        </>
              }
    };

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
        style={if *view_state == ViewState::Exif {
          format!("background-image: url({}", "/assets/demo.jpg")
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
            <div class="flex min-w-full md:min-w-[80%] max-h-[100%]
            flex-col items-center
            drop-shadow-md 
            bg-white bg-opacity-95 
            my-10 
            p-8 md:p-12 rounded-xl md:rounded-3xl 
            overflow-hidden
            ease ">
              {match *view_state {
                ViewState::Add => render_add,
                ViewState::Exif => render_exif,
                }
              }

            </div>
        </div>
    }
}
