use std::{collections::HashMap, rc::Rc};

use exif::Tag;
use yew::prelude::*;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
    pub exif: HashMap<Tag, String>,
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum FileError {
    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Can't get data from file: {0}")]
    InvalidData(String),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Message (Action)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Msg {
    Loaded(Result<FileDetails, FileError>),
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
            Msg::Clear => None,
        };

        Self { file }.into()
    }
}

pub type AppContext = UseReducerHandle<AppState>;

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
