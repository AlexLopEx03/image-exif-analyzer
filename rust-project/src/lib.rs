mod exif;

use wasm_bindgen::prelude::wasm_bindgen;
use serde_json::to_string;

#[wasm_bindgen]
pub fn extract_metadata(image: &[u8]) -> String{
    let image_metadata = exif::parse_exif_metadata(&image);
    to_string(&image_metadata).unwrap_or_else(|_| "{}".to_string())
}