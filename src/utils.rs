use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use img_parts::{Bytes, DynImage, ImageEXIF};

use crate::types::{ExifMap, FileDetails, FileError};

pub fn img_from_bytes(data: Vec<u8>) -> Result<DynImage, FileError> {
    

    DynImage::from_bytes(data.clone().into())
        .map_err(|e| FileError::InvalidData(e.to_string()))?
        .ok_or_else(|| FileError::InvalidData("No image data".to_string()))
}

pub fn exif_to_map(bytes: Bytes) -> Result<ExifMap, FileError> {
    let mut exif_map: ExifMap = HashMap::new();

    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_raw(bytes.to_vec())
        .map_err(|e| FileError::InvalidExif(e.to_string()))?;

    for f in exif.fields() {
        exif_map.insert(f.tag, f.display_value().to_string());
    }

    Ok(exif_map)
}

pub fn get_file_details(
    data: Vec<u8>,
    file_name: String,
    file_type: String,
) -> Result<FileDetails, FileError> {
    let image = img_from_bytes(data.clone())?;

    let exif_map = match image.exif() {
        Some(exif) => exif_to_map(exif)?,
        // No exif data means empty map
        None => HashMap::new(),
    };

    Ok(FileDetails {
        name: file_name,
        file_type,
        data,
        exif: exif_map,
    })
}

pub fn remove_exif(details: FileDetails) -> Result<FileDetails, FileError> {
    let FileDetails { data, .. } = details;
    let mut image = img_from_bytes(data.clone())?;
    image.set_exif(None);
    let data = image.encoder().bytes().into();

    Ok(FileDetails {
        name: details.name,
        file_type: details.file_type,
        data,
        exif: HashMap::new(),
    })
}

pub fn img_src(file: &FileDetails) -> String {
    format!(
        "data:{};base64,{}",
        file.file_type,
        STANDARD.encode(&file.data)
    )
}

pub fn exified_file_name(file: &FileDetails) -> String {
    format!("exified-{}", &file.name)
}
