use crate::core::LennaImage;
use image;
use img_parts::{jpeg::Jpeg, png::Png, ImageEXIF};

use exif::experimental::Writer as ExifWriter;

pub fn write_to_file(
    image: &LennaImage,
    format: image::ImageOutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(&image.path);
    let path = path.join(format!(
        "{}.{}",
        image.name,
        match format {
            image::ImageOutputFormat::Png => "png",
            _ => "jpg",
        }
    ));

    let img = &image.image;

    match image.exif_out.len() {
        0 => {
            img.save(path).unwrap();
            Ok(())
        }
        _ => {
            let mut file_data: Vec<u8> = Vec::new();
            match img.write_to(&mut file_data, format.clone()) {
                Ok(_) => (),
                Err(_) => (),
            };

            let mut exif_writer = ExifWriter::new();

            let exif_fields = image.exif_out.to_vec();
            for f in &exif_fields {
                exif_writer.push_field(&f);
            }
            let mut exif_data = std::io::Cursor::new(Vec::new());
            exif_writer.write(&mut exif_data, false).unwrap();

            let output = std::fs::File::create(path)?;

            match format {
                image::ImageOutputFormat::Png => {
                    let mut png = Png::from_bytes(file_data.into()).unwrap();
                    png.set_exif(Some(exif_data.into_inner().into()));
                    png.encoder().write_to(output)?;
                }
                _ => {
                    let mut jpeg = Jpeg::from_bytes(file_data.into()).unwrap();
                    jpeg.set_exif(Some(exif_data.into_inner().into()));
                    jpeg.encoder().write_to(output)?;
                }
            };

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::read::read_from_file;
    use exif::{Field, In, Tag, Value};

    #[test]
    fn write_file() {
        let mut image = read_from_file("lenna.png".into()).unwrap();
        image.name = "lenna_test_write".to_string();

        write_to_file(&image, image::ImageOutputFormat::Jpeg(90)).unwrap();

        let image_desc = Field {
            tag: Tag::ImageDescription,
            ifd_num: In::PRIMARY,
            value: Value::Ascii(vec![b"Write Test".to_vec()]),
        };
        image.exif_out.push(image_desc);

        write_to_file(&image, image::ImageOutputFormat::Jpeg(90)).unwrap();

        let image = read_from_file("lenna_test_write.jpg".into()).unwrap();
        assert_eq!(image.exif_out.len(), 1);
        let read_desc = image
            .exif
            .get_field(Tag::ImageDescription, In::PRIMARY)
            .unwrap();
        assert_eq!(read_desc.display_value().to_string(), "\"Write Test\"");
    }
}
