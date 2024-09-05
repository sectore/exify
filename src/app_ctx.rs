use std::rc::Rc;

use yew::prelude::*;

use crate::{
    types::{AppContext, FileDetails, FileError, FileResult},
    utils::remove_exif,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Message (Action)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Msg {
    Update(Result<FileDetails, FileError>),
    RemoveExif,
    Saved(Result<String, FileError>),
    Clear,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  App state
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct AppState {
    pub file: Option<FileResult>,
    pub exified: bool,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Reducer
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Reducible for AppState {
    type Action = Msg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let state: AppState = match action {
            Msg::Update(details) => AppState {
                file: Some(details),
                exified: self.exified,
            },
            Msg::RemoveExif => {
                if let Some(Ok(details)) = &self.file {
                    let result = remove_exif(details.clone());

                    AppState {
                        file: Some(result.clone()),
                        exified: result.is_ok(),
                    }
                } else {
                    AppState {
                        file: None,
                        exified: false,
                    }
                }
            }
            Msg::Saved(_) => AppState {
                file: None,
                exified: false,
            },
            Msg::Clear => AppState {
                file: None,
                exified: false,
            },
        };

        state.into()
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct AppProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppProvider(props: &AppProviderProps) -> Html {
    let msg = use_reducer(AppState::default);

    html! {
        <ContextProvider<AppContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<AppContext>>
    }
}
