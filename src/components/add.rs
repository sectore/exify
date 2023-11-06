use std::cell::RefCell;
use std::rc::Rc;

use gloo::console::log;
use gloo::file::callbacks::FileReader;
use yew::prelude::*;

use crate::icons::{Image, Logo, Plus};

use crate::app_ctx::Msg;
use crate::components::download::Download;
use crate::types::{AppContext, FileError};
use crate::utils::get_file_details;

use gloo::file::File;
use yew::html::TargetCast;
use yew::{html, Callback, Html};

use web_sys::{DragEvent, FileList, HtmlInputElement};

#[function_component]
pub fn Add() -> Html {
    let ctx = use_context::<AppContext>().unwrap();

    let task_ref: Rc<RefCell<Option<FileReader>>> = use_mut_ref(|| None);

    let ondragstate = use_state(|| false);

    let files_selected = {
        let ctx = ctx.clone();
        let task_ref = task_ref.clone();
        log!("files_selected called");
        Callback::from(move |files: FileList| {
            log!("files_selected: {:?}", files.length());
            match files.item(0) {
                None => {
                    return ctx.dispatch(Msg::Loaded(Err(FileError::InvalidData(
                        "No file in FileList".to_string(),
                    ))));
                }
                Some(file) => {
                    let file = File::from(web_sys::File::from(file));
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();
                    log!("file type: {:?}", &file_type);
                    let ctx = ctx.clone();
                    let task = gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        log!("file loaded");

                        let msg = match res {
                            Ok(data) => {
                                let file_details = get_file_details(data, file_name, file_type);
                                file_details
                            }
                            Err(e) => Err(FileError::InvalidData(e.to_string())),
                        };

                        ctx.dispatch(Msg::Loaded(msg));
                    });

                    // store task so it doesn't get dropped
                    *task_ref.borrow_mut() = Some(task);
                }
            };
        })
    };

    let onchange = {
        let files_selected = files_selected.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.files().and_then(|list| {
                log!("files selected: {:?}", list.length());
                files_selected.emit(list);
                return Some(true);
            });
        })
    };

    let ondrop = {
        let s = ondragstate.clone();

        Callback::from(move |event: DragEvent| {
            event.prevent_default();

            event
                .data_transfer()
                .and_then(|data| data.files())
                .and_then(|list| {
                    files_selected.emit(list);
                    return Some(true);
                });
            s.set(false)
        })
    };

    let ondragover = {
        let s = ondragstate.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            s.set(true);
        })
    };

    let ondragleave = {
        let s = ondragstate.clone();

        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            s.set(false);
        })
    };

    let ondragenter = {
        let s = ondragstate.clone();

        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            s.set(false);
        })
    };

    html! {

    <div
      class={classes!("flex", "flex-col",
        "items-center", "justify-center",
        "w-full", "h-full",
      )}
      {ondrop}
      {ondragover}
      {ondragleave}
      {ondragenter}
    >
          <Image class={classes!(
            "w-36", "h-36", "sm:w-56", "sm:h-56",
            "mb-2", "sm:mb-4",
            "text-sky-600",
            "text-shadow-light",
            "ease",
            ondragstate.then(|| "text-sky-500 scale-105")
          )}
          />
          <p class={classes!(
              "text-sky-600", "font-bold", "text-center", "text-2xl",
              "sm:text-4xl", "uppercase",
              "text-shadow-light",
              "ease",
              ondragstate.then(|| "group-hover:text-sky-500 group-hover:scale-105")
            )}
            >{"Drop image here"}</p>
          <p class="
              text-gray-300 font-bold text-center text-xl sm:text-2xl uppercase 
              text-shadow-light 
              mt-2 sm:mt-6 mb-4 sm:mb-8"
            >{"or"}</p>
            <label
              for="img-upload"
              class="btn
              pl-4 sm:pl-10 pr-2 sm:pr-4 mb-3
              w-full sm:w-auto
              "
            >
            {"Select image"}
            <Plus class="w-8 h-8 sm:w-12 sm:h-12 ml-2 sn:ml-4" />
          </label>
          <p class="text-gray-300 text-sm sm:text-base text-shadow-light">{"Supports jpg, png, webp"}</p>
          <input id="img-upload" type="file" class="hidden" accept="image/*" {onchange} />
          // <Spinner class="w-10 h-10 text-sky-300 mt-10" />
    </div>

      }
}
