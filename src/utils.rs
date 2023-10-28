use std::collections::HashMap;

use gloo::console::log;

use img_parts::{Bytes, DynImage, ImageEXIF};

use crate::types::{ExifMap, FileDetails, FileError};

pub fn img_from_bytes(data: Vec<u8>) -> Result<DynImage, FileError> {
    let image = DynImage::from_bytes(data.clone().into())
        .map_err(|e| FileError::InvalidData(e.to_string()))?
        .ok_or_else(|| FileError::InvalidData("No image data".to_string()));

    image
}

pub fn exif_to_map(bytes: Bytes) -> Result<ExifMap, FileError> {
    let mut exif_map: ExifMap = HashMap::new();

    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_raw(bytes.to_vec())
        .map_err(|e| FileError::InvalidExif(e.to_string()))?;

    for f in exif.fields() {
        log!(
            "f {:?} {:?}",
            f.tag.to_string(),
            f.display_value().to_string()
        );
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

    let exif_data = image
        .exif()
        .ok_or(FileError::InvalidData("No exif data".to_string()))?;

    let exif_map = exif_to_map(exif_data)?;

    Ok(FileDetails {
        name: file_name,
        file_type,
        data,
        exif: exif_map,
    })
}
