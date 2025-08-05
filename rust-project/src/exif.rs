use std::collections::HashMap;
use std::io::Cursor;
use serde::Serialize;
use exif::Reader;

#[derive(Serialize)]
pub struct ExifMetadata {
    pub fields: HashMap<String, String>,
}

pub fn parse_exif_metadata(image: &[u8]) -> ExifMetadata{

    let mut cursor = Cursor::new(image);
    let reader = Reader::new();

    let exif = match reader.read_from_container(&mut cursor) {
        Ok(exif) => exif,
        Err(error) => {
            eprint!("{}", error);
            return ExifMetadata {
                fields: HashMap::from([
                    ("error".to_string(), "No EXIF data available".to_string())           
                ])
            };
        }
    };
    let mut fields = HashMap::new();
    for field in exif.fields(){
        let tag = field.tag.to_string();
        let value = field.display_value().with_unit(&exif).to_string();
        fields.insert(tag, value);
    }
    ExifMetadata { fields }
}