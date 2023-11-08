use gloo::utils::document;
use wasm_bindgen::JsValue;
use web_sys::{HtmlAnchorElement, Url};
use yew::prelude::*;

use crate::icons::{BrokenImage, Close};
use crate::types::{AppContext, FileError};
use crate::utils::{img_src, exified_file_name};
use crate::app_ctx::Msg;

use yew::html;


#[function_component]
pub fn Details() -> Html {

    let ctx = use_context::<AppContext>().unwrap();

    let file_details = use_memo(ctx.file.clone(), |file| file.clone().and_then(Result::ok));

    let file_error = use_memo(ctx.file.clone(), |file| file.clone().and_then(Result::err));

    let is_exified = use_memo(ctx.exified, |ex| ex.clone());

    let has_exif = use_memo(ctx.file.clone(), |file| 
      file.clone().and_then(Result::ok).map(|fd| !fd.exif.is_empty()).unwrap_or(false));

    let on_remove = {
        let ctx = ctx.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let ctx = ctx.clone();

            ctx.dispatch(Msg::RemoveExif);
        })
    };

    let on_save = {
      let fd = file_details.clone();
      let ctx = ctx.clone();
      Callback::from(move |_: MouseEvent| {
            if let Some(fd) = &*fd {
                // transform Vec<u8> into JSValue (Array)
                let u8_array =
                    js_sys::Array::of1(&js_sys::Uint8Array::from(&fd.data[..]));
                // get URL (blob) to download file
                let result = web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &u8_array,
                    web_sys::BlobPropertyBag::new().type_(&fd.file_type),
                )
                .and_then(|blob| web_sys::Url::create_object_url_with_blob(&blob))
                .and_then(|url| 
                    // Create <a> element to download file
                    web_sys::window()
                      .and_then(|w| w.document())
                      // Map error needed to stay with Result<_, JSValue>
                      .ok_or(JsValue::from_str("no document"))
                      .and_then(|d| d.create_element("a"))
                      .and_then(|elem| {
                        let name = exified_file_name(&fd);
                          let a: HtmlAnchorElement = HtmlAnchorElement::from(JsValue::from(elem));
                          a.set_href(&url);
                          a.set_download(&name);
                          a.set_class_name("hidden");
                          a.click();
                          // cleanup
                          Url::revoke_object_url(&url).unwrap();
                          document().body().unwrap().remove_child(&a).unwrap();
                          // return name
                          Ok(name)
                        })
                        
                  );

                  let result = result.map_err(|e| FileError::SaveFailed(
                    e.as_string()
                    .unwrap_or("failed to save file".to_owned())
                  ));
                  ctx.dispatch(Msg::Saved(result));
          };
        })
    };

    let on_cancel = {
        let ctx = ctx.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let ctx = ctx.clone();

            ctx.dispatch(Msg::Clear);
        })
    };

    html! {
      <>
      <div class="absolute right-8 md:right-10 top-8 md:top-10 w-12 h-12 md:w-14 md:h-14" onclick={on_cancel}>
        <Close class="w-full h-full" />
      </div>  
      
      { if let Some(fd) = &*file_details {
        html!{
          <div class="flex flex-col w-full items-center">
            <img
              class="max-w-[10rem] max-h-[10rem] w-auto h-auto border-[1em] border-sky-600 "
              src={img_src(&fd)} />
            <p class="text-gray-400 text-sm md:text-base mt-2 truncate">
              { if *is_exified {
                  exified_file_name(&fd)
                } else {
                  fd.name.clone()
                }
              }
            </p>
          </div>
          }
        } else {
          html!{
            <BrokenImage class="!w-56 !h-56 text-gray-300 " />
          }
        }
      }

      {
        if *is_exified {
          html!{
            <button class="btn px-10 lg:px-28 mt-8 lg:mt-12
            w-full lg:w-auto" onclick={on_save}>{"Save"}</button>
          }
        } else if file_details.is_some() && *has_exif {
          html!{
            <button class="btn px-10 lg:px-28 mt-8 lg:mt-12
            w-full lg:w-auto"
              onclick={on_remove}>{"Remove EXIF"}</button>
          }
        } else {
          html!{}
        }
      }

          // error message
          { if let Some(err) = &*file_error {
              html!{
                <div class="w-full h-full flex flex-col items-center justify-center">
                  <h2 class="text-2xl font-bold text-gray-400 py-2 uppercase">
                    {"Error"}</h2>
                  <p class="text-xl text-gray-400">{err.to_string()}</p>
                </div>
              }
            } else {
              html!{}
            }
          }

          <h2 class="text-xl md:text-2xl font-bold text-gray-400 my-8 ">
          {
            if *is_exified {
              "EXIF data removed".to_owned()
            } else if let Some(fd) = &*file_details {
              if fd.exif.is_empty() {
                "EXIF data not found".to_owned()
              } else {
                format!("{:?} EXIF data found", &fd.exif.len())
              }
            } else {
              "".to_owned()
            }
          }
          </h2>

          {
            if let Some(fd) = &*file_details {
              if !fd.exif.is_empty() {
              html!(
                <>
                  <div class="w-full flex flex-row justify-center
                  text-gray-500 bg-gray-200 
                  text-shadow-light
                  text-xs md:text-base">
                      <div class="w-1/2 md:w-1/3 px-3 py-1 border-r border-white">{"Name"}</div>
                      <div class="w-1/2 md:w-2/3 px-3 py-1">{"Data"}</div>
                  </div>
                  <div class="w-full overflow-y-scroll">
                  { for fd.exif.iter().map(|(k, v)| html! {
                      <div class="w-full flex justify-center
                      text-xs md:text-base  text-gray-500 text-shadow-light
                      odd:bg-gray-100">
                        <div class="w-1/2 md:w-1/3 px-3 py-1 border-r border-gray-200 truncate">{k.to_string()}</div>
                        <div class="w-1/2 md:w-2/3 px-3 py-1 truncate">{v.to_string()}</div>
                      </div>
                    })}
                  </div>
                </>
              )
            } else { html!{} }
          } else { html!{} }
          }
        </>
    }
}
