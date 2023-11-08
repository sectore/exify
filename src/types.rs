use std::collections::HashMap;

use exif::Tag;
use yew::UseReducerHandle;

use thiserror::Error;

use crate::app_ctx::AppState;

pub type ExifMap = HashMap<Tag, String>;

pub type AppContext = UseReducerHandle<AppState>;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum FileError {
    #[error("Invalid exif data: {0}")]
    InvalidExif(String),

    #[error("Can't get data from file: {0}")]
    InvalidData(String),

    #[error("Drag & drop image failed: {0}")]
    DragDropFailed(String),

    #[error("Couldn't save file: {0}")]
    SaveFailed(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
    pub exif: ExifMap,
}

pub type FileResult = Result<FileDetails, FileError>;
