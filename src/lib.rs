use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::fs::File;
use std::string::ToString;

fn empty_string() -> String {
    "".to_string()
}

fn empty_int_as_string() -> String {
    "0".to_string()
}

fn empty_float() -> f32 {
    0.0
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeData {
    #[serde(default = "empty_int_as_string")]
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

    #[serde(alias = "imageViews", default = "empty_int_as_string")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AlbumData {
    #[serde(default = "empty_string")]
    pub title: String,

    #[serde(default = "empty_string")]
    pub description: String,

    #[serde(default = "empty_string")]
    pub access: String,

    #[serde(default = "empty_string")]
    pub location: String,

    #[serde(alias = "date")]
    pub date: TimeData,

    #[serde(alias = "geoData")]
    pub geo_data: GeoData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AlbumMetaDataInformation {
    #[serde(alias = "albumData")]
    pub album_data: AlbumData,
}

impl PhotoMetaInformation {
    pub fn from_file(input_file: &str) -> Result<PhotoMetaInformation, ()> {
        let file_handle = File::open(input_file);
        if file_handle.is_ok() {
            match serde_json::from_reader(file_handle.unwrap()) {
                Ok(meta) => return Ok(meta),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }
}

impl AlbumMetaDataInformation {
    pub fn from_file(input_file: &str) -> Result<AlbumMetaDataInformation, ()> {
        let file_handle = File::open(input_file);
        if file_handle.is_ok() {
            match serde_json::from_reader(file_handle.unwrap()) {
                Ok(meta) => return Ok(meta),
                Err(err) => return Err(()),
            }
        }
        Err(())
    }
}
