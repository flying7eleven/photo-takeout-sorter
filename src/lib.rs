use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::fs::File;
use std::string::ToString;

fn empty_string() -> String {
    "".to_string()
}

fn empty_int() -> String {
    "0".to_string()
}

fn empty_float() -> f32 {
    0.0
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeData {
    #[serde(default = "empty_int")]
    pub timestamp: String,

    #[serde(default = "empty_string")]
    pub formatted: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoData {
    #[serde(default = "empty_float")]
    pub latitude: f32,

    #[serde(default = "empty_float")]
    pub longitude: f32,

    #[serde(default = "empty_float")]
    pub altitude: f32,

    #[serde(alias = "latitudeSpan", default = "empty_float")]
    pub latitude_span: f32,

    #[serde(alias = "longitudeSpan", default = "empty_float")]
    pub longitude_span: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoMetaInformation {
    #[serde(default = "empty_string")]
    pub title: String,

    #[serde(default = "empty_string")]
    pub description: String,

    #[serde(alias = "imageViews", default = "empty_int")]
    pub image_views: String,

    #[serde(alias = "creationTime")]
    pub creation_time: TimeData,

    #[serde(alias = "modificationTime")]
    pub modification_time: TimeData,

    #[serde(alias = "geoData")]
    pub geo_data: GeoData,

    #[serde(alias = "geoDataExif")]
    pub geo_data_exif: GeoData,

    #[serde(alias = "photoTakenTime")]
    pub photo_taken_time: TimeData,
}

impl PhotoMetaInformation {
    pub fn from_file(input_file: &str) -> Result<PhotoMetaInformation, ()> {
        let file_handle = File::open(input_file);
        if file_handle.is_ok() {
            let read_meta_data: PhotoMetaInformation =
                serde_json::from_reader(file_handle.unwrap()).unwrap();
            return Ok(read_meta_data);
        }
        Err(())
    }
}
