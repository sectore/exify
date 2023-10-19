use std::cell::RefCell;
use std::rc::Rc;

use gloo::console::log;
use gloo::file::callbacks::FileReader;
use yew::prelude::*;

use crate::app_ctx::{AppContext, FileDetails, FileError, Msg};

use gloo::file::File;

use yew::html::TargetCast;
use yew::{html, Callback, Html};

use web_sys::{DragEvent, FileList, HtmlInputElement};

#[function_component]
pub fn Upload() -> Html {
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
                    ctx.dispatch(Msg::Loaded(Err(FileError::InvalidData(
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
                        match res {
                            Ok(_) => ctx.dispatch(Msg::Loaded(Ok(FileDetails {
                                name: file_name,
                                file_type,
                                data: res.expect("failed to read file"),
                            }))),
                            Err(e) => ctx
                                .dispatch(Msg::Loaded(Err(FileError::InvalidData(e.to_string())))),
                        };
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

      <div class="flex items-center justify-center w-5/6">
      <label
      for="img-upload"
      class={classes!("flex", "flex-col",
      "items-center", "justify-center",
      "w-full", "h-64",
      "border-2", "border-gray-300", "hover:border-sky-500", "border-dashed",
      "rounded-lg", "cursor-pointer", "bg-gray-50", "hover:bg-gray-100",
      ondragstate.then(|| "bg-sky-100 border-sky-500")
    )}
      {ondrop}
      {ondragover}
      {ondragleave}
      {ondragenter}
      >
          <div
          class="flex flex-col items-center justify-center pt-5 pb-6"
          >
              <svg class="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"/>
              </svg>
              <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">{"Click to upload"}</span>{" or drag and drop"}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">{"Images only (PNG, JPG)"}</p>
          </div>
          <input id="img-upload" type="file" class="hidden" accept="image/*" {onchange} />
      </label>
      <div>{ondragstate.to_string()}</div>
      </div>

          }
}
