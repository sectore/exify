use std::rc::Rc;

use yew::prelude::*;

use crate::types::{AppContext, FileDetails, FileError};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Message (Action)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Msg {
    Loaded(Result<FileDetails, FileError>),
    RemoveExif,
    Clear,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  App state
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppState {
    pub file: Option<Result<FileDetails, FileError>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { file: None }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Reducer
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Reducible for AppState {
    type Action = Msg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let file = match action {
            Msg::Loaded(details) => Some(details),
            Msg::RemoveExif => None,
            // {
            // let details = self.file.map(|a| {
            //     a.map(|b| {
            //         let exif = HashMap::new();
            //         let o_data = match b.format {
            //             image::ImageFormat::Jpeg => {
            //                 let img = Jpeg::from_bytes(b.data.clone().into()).ok();
            //                 img.map(|e| e.set_exif(None));
            //                 img
            //             }
            //             _ => None,
            //         };

            //         let details = o_data.map(|data| FileDetails {
            //             exif,
            //             data: data.encoder().bytes().into(),
            //             ..b
            //         });
            //         details
            //     })
            // });
            // details
            // }
            Msg::Clear => None,
        };

        Self { file }.into()
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct AppProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppProvider(props: &AppProviderProps) -> Html {
    let msg = use_reducer(|| AppState::default());

    html! {
        <ContextProvider<AppContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<AppContext>>
    }
}
