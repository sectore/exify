use std::rc::Rc;

use yew::prelude::*;

use crate::{
    types::{AppContext, FileDetails, FileError},
    utils::remove_exif,
};

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
            Msg::RemoveExif => {
                if let Some(Ok(details)) = &self.file {
                    Some(remove_exif(details.clone()))
                } else {
                    None
                }
            }
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
