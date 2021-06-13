use crate::core::LennaImage;
use exif::{experimental::Writer as ExifWriter, Field, Tag};
use image;
use img_parts::{jpeg::Jpeg, png::Png, ImageEXIF, ImageICC};
use std::io::{Cursor, Seek, SeekFrom};

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
    let image_data = write_to_data(image, format)?;

    std::fs::write(path, image_data).unwrap();
    Ok(())
}

pub fn write_to_data(
    image: &LennaImage,
    format: image::ImageOutputFormat,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = &image.image;
    let mut image_data: Vec<u8> = Vec::new();

    img.write_to(&mut image_data, format.clone()).unwrap();
    let mut thumbnail = Vec::<u8>::new();

    match image.exif.len() {
        0 => Ok(image_data),
        _ => {
            let mut exif_writer = ExifWriter::new();

            let exif_fields = image.exif.to_vec();

            for f in &exif_fields {
                match *f {
                    Field {
                        tag: Tag::JPEGInterchangeFormat,
                        ifd_num: exif::In::THUMBNAIL,
                        ..
                    } => {
                        if let exif::Value::Byte(data) = &f.value {
                            thumbnail = data.to_vec();
                        }
                    }
                    _ => exif_writer.push_field(&f),
                };
            }
            if !thumbnail.is_empty() {
                exif_writer.set_jpeg(&thumbnail[..], exif::In::THUMBNAIL);
            }

            let mut exif_data = Cursor::new(Vec::new());
            exif_writer.write(&mut exif_data, false).unwrap();
            exif_data.seek(SeekFrom::Start(0)).unwrap();

            let mut out = Vec::new();
            match format {
                image::ImageOutputFormat::Png => {
                    let mut png = Png::from_bytes(image_data.into()).unwrap();
                    png.set_exif(Some(exif_data.into_inner().into()));
                    png.encoder().write_to(&mut out)?;
                }
                _ => {
                    let mut jpeg = Jpeg::from_bytes(image_data.into()).unwrap();
                    jpeg.set_exif(Some(exif_data.into_inner().into()));
                    jpeg.set_icc_profile(None);
                    jpeg.encoder().write_to(&mut out)?;
                }
            };
            Ok(out)
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
        image.exif.push(image_desc);

        write_to_file(&image, image::ImageOutputFormat::Jpeg(90)).unwrap();

        let image = read_from_file("lenna_test_write.jpg".into()).unwrap();
        assert_eq!(image.exif.len(), 1);
        let read_desc = image.exif.get(0).unwrap();
        assert_eq!(read_desc.display_value().to_string(), "\"Write Test\"");
    }
}
