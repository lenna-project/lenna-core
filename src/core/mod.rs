//! core functionalities
pub mod config;
pub mod pipeline;
pub mod pool;
pub mod processor;
pub mod resize;

use exif::Field;
use image::DynamicImage;

/// image struct
pub struct LennaImage {
    /// name of image, also used as filename
    pub name: String,
    /// image data container
    pub image: Box<DynamicImage>,
    /// path, also folder for image when saved
    pub path: String,
    /// image meta data
    pub exif: Box<Vec<Field>>,
}

impl Default for LennaImage {
    fn default() -> Self {
        LennaImage {
            name: "unnamed".to_string(),
            image: Box::new(DynamicImage::new_rgb8(1, 1)),
            path: "".to_string(),
            exif: Box::new(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lenna_image() {
        let image = LennaImage::default();
        assert_eq!(image.name, "unnamed");
    }
}
