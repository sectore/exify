use std::cell::RefCell;
use std::rc::Rc;

use gloo::file::callbacks::FileReader;
use yew::prelude::*;

use crate::icons::{Image, Plus};

use crate::app_ctx::Msg;
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

    let ondragstate = use_state_eq(|| false);

    let files_selected = {
        let ctx = ctx.clone();
        let task_ref = task_ref.clone();
        Callback::from(move |files: FileList| {
            match files.item(0) {
                None => {
                    return ctx.dispatch(Msg::Update(Err(FileError::DragDropFailed(
                        "No file in FileList".to_string(),
                    ))));
                }
                Some(file) => {
                    let file = File::from(web_sys::File::from(file));
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();
                    let ctx = ctx.clone();
                    let task = gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        let msg = match res {
                            Ok(data) => {
                                let file_details = get_file_details(data, file_name, file_type);
                                file_details
                            }
                            Err(e) => Err(FileError::InvalidData(e.to_string())),
                        };

                        ctx.dispatch(Msg::Update(msg));
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
                files_selected.emit(list);
                return Some(true);
            });
        })
    };

    let ondrop = {
        let s = ondragstate.clone();
        Callback::from(move |event: DragEvent| {
            // Prevent default behavior (Prevent file from being opened)
            event.prevent_default();
            event.stop_propagation();

            s.set(false);

            event
                .data_transfer()
                .and_then(|data| {
                    return data.files();
                })
                .and_then(|list| {
                    files_selected.emit(list.clone());
                    return Some(true);
                });
        })
    };

    let ondragover = {
        let s = ondragstate.clone();
        Callback::from(move |event: DragEvent| {
            // Prevent default behavior (Prevent file from being opened)
            // https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API/File_drag_and_drop#prevent_the_browsers_default_drag_behavior
            event.prevent_default();
            event.stop_propagation();

            let _ = event.data_transfer().and_then(|_| {
                return Some(true);
            });

            s.set(true);
        })
    };

    let ondragleave = {
        let s = ondragstate.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            event.stop_propagation();
            s.set(false);
        })
    };

    let ondragenter = {
        let s = ondragstate.clone();

        Callback::from(move |event: DragEvent| {
            event.prevent_default();

            let _ = event.data_transfer().and_then(|_| {
                return Some(true);
            });

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
            "w-56", "h-56",
            "mb-2", "sm:mb-4",
            "text-sky-600",
            "text-shadow-light",
            "ease",
            ondragstate.then(|| "text-sky-500 scale-110")
          )}
          />
          <p class={classes!(
              "text-sky-600", "font-bold", "text-center",
              "text-3xl", "md:text-4xl", "uppercase",
              "text-shadow-light",
              "ease",
              ondragstate.then(|| "text-sky-500 scale-105")
            )}
            >
            <span class="hidden md:block">{"Drop your image here"}</span>
            <span class="md:hidden">{"Drop image"}</span>
            </p>
          {if let Some(Err(FileError::DragDropFailed(e))) = ctx.file.clone() {
            html!{<p class="my-1 text-lg text-red-500 font-normal">
            {format!("ERROR {:?}", e)}</p>}
            } else {
              html!{}
            }
          }
          <p class="
              text-gray-300 font-bold text-center text-2xl uppercase 
              text-shadow-light 
              mt-2 sm:mt-6 mb-4 sm:mb-8"
            >{"or"}</p>
            <label
              for="img-upload"
              class="btn
              pl-10 pr-6 mb-3
              w-auto
              "
            >
            <span class="md:hidden">{"Select"}</span>
            <span class="hidden md:block">{"Select image"}</span>
            <Plus class="w-8 h-8 md:w-12 md:h-12 ml-2 md:ml-4" />
          </label>
          <p class="text-gray-400 text-base text-shadow-light">{"Supports jpg, png, webp"}</p>
          <input id="img-upload" type="file" class="hidden" accept="image/*" {onchange} />
    </div>

      }
}
