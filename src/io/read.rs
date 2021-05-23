use crate::core::LennaImage;

use exif::{Field, Reader as ExifReader};
use std::io::Read;

pub fn read_from_file(path: String) -> Result<LennaImage, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(&path);

    let mut file = std::fs::File::open(path)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let mut image = read_from_data(data).unwrap();
    image.name = path.file_stem().unwrap().to_str().unwrap().to_string();
    image.path = path.parent().unwrap().to_str().unwrap().to_string();
    Ok(image)
}

pub fn read_from_data(data: Vec<u8>) -> Result<LennaImage, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(&data).unwrap();

    let mut c = std::io::Cursor::new(&data);
    let exif = match ExifReader::new().read_from_container(&mut c) {
        Ok(exif) => exif,
        Err(_) => ExifReader::new()
            .read_raw(b"MM\0\x2a\0\0\0\x08\0\0\0\0\0\0".to_vec())
            .unwrap(),
    };
    let mut exif_out: Vec<Field> = Vec::new();
    for f in exif.fields() {
        exif_out.push(f.clone());
    }

    Ok(LennaImage {
        name: "".to_string(),
        path: "".to_string(),
        image: Box::new(img),
        exif: Box::new(exif_out),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let image = read_from_file("lenna.png".into()).unwrap();
        assert_eq!(image.exif.len(), 0);
        assert_eq!(image.name, "lenna".to_string());
        assert_eq!(image.path, "".to_string());
    }
}
