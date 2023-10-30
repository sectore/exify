
use wasm_bindgen::JsValue;
use web_sys::{HtmlAnchorElement, Url};
use yew::prelude::*;

use gloo::utils::document;

use crate::types::AppContext;


#[function_component(Download)]
pub fn download() -> Html {

    let error_msg: UseStateHandle<Option<String>> = use_state(|| None);

    let ctx = use_context::<AppContext>().unwrap();


    let upload_file = {
      let error_msg = error_msg.clone();
      Callback::from(move |_: MouseEvent| {
            if let Some(Ok(file_details)) = &ctx.file {
                // transform Vec<u8> into JSValue (Array)
                let u8_array =
                    js_sys::Array::of1(&js_sys::Uint8Array::from(&file_details.data[..]));
                // get URL (blob) to download file
                let r = web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &u8_array,
                    web_sys::BlobPropertyBag::new().type_(&file_details.file_type),
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
                          let a: HtmlAnchorElement = HtmlAnchorElement::from(JsValue::from(elem));
                          a.set_href(&url);
                          a.set_download(format!("exified-{}", &file_details.name).as_str());
                          a.set_class_name("hidden");
                          a.click();
                          Ok(a)
                        })
                        .and_then(|a| {
                          // cleanup
                          Url::revoke_object_url(&url).unwrap();
                          document().body().unwrap().remove_child(&a).unwrap();
                          // return something
                          Ok(true)
                        })
                      
                  );       

                  if let Err(e) = r {
                    error_msg.set(Some(format!("Error {:?}", e)));
                  }

          };
        })
    };

    html! {
      <div>
        <button onclick={upload_file}>{"Download"}</button>
        {
          if let Some(e) = &*error_msg {
            html!(<p>{format!("Error downloading file: {:?}", e)}</p>)
          } else {
            html!{}
          }
        }

      </div>
    }
  }
