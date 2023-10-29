use std::cell::RefCell;
use std::rc::Rc;

use gloo::console::{error, log};
use gloo::file::callbacks::FileReader;
use yew::prelude::*;

use crate::app_ctx::Msg;
use crate::types::{AppContext, FileError};
use crate::utils::get_file_details;

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

    <div class="flex items-center justify-center w-5/6">
    <label
    for="img-upload"
    class={classes!("flex",
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
        class="flex items-center justify-center pt-5 pb-6"
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
    <button onclick={on_remove}>{"Remove exif"}</button>
    <button onclick={on_clear}>{"Clear data"}</button>
    <div>
    {
      if let Some(Ok(file_details)) = &ctx.file {
        let u8_array = js_sys::Array::of1(&js_sys::Uint8Array::from(&file_details.data[..]));

        if let Ok(data_url) = web_sys::Blob::new_with_u8_array_sequence_and_options(
            &u8_array,
            web_sys::BlobPropertyBag::new().type_(&file_details.file_type),
          ).and_then(|blob| web_sys::Url::create_object_url_with_blob(&blob)) {

            html!{
              <a download={format!("exified-{}", file_details.name.clone())} target="_blank" href={data_url.clone()}
                onclick={Callback::from(move |_: MouseEvent| {
                  if let Err(e) = web_sys::Url::revoke_object_url(&data_url) {
                    error!(format!("failed to revoke object url: {:?}", e));
                } else {
                    log!("revoked object url");
                }})}>{"Download"}</a>
              }
          } else {
            html!{}
          }
      }
      else {
        html! {}
      }
    }
    </div>

    </div>

        }
}
