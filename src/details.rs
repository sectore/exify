use yew::prelude::*;

use crate::types::FileDetails;
use crate::utils::img_src;

use yew::{classes, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub file_details: FileDetails,
}

#[function_component(Details)]
pub fn details(props: &Props) -> Html {
    let Props {
        class,
        file_details,
    } = props;

    html! {
        <div class={classes!(
          "flex", "flex-col", "items-center", "bg-white", "opacity-75", "rounded-2xl", "shadow-2xl",
          class.clone(),
        )}
        >
          <img
            class="max-w-28 max-h-28"
            src={img_src(&file_details)} />
          <div class="w-full p-6">
          {
            if &file_details.exif.len() <= &0 {
              html! { <h2 class="text-xl">{"no exif data"}</h2> }
            } else {
              html! {
                <>
              <h2 class="text-xl">{format!("Exif data: {:?}", &file_details.exif.len())}</h2>
              <table class="">
                { for file_details.exif.iter().map(|(k, v)| html! {
                  <tr>
                    <td>{k.to_string()}</td>
                    <td>{v.to_string()}</td>
                  </tr>
                })}
              </table>
              </>
              }
            }
          }
          </div>
        </div>
    }
}
