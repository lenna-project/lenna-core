pub mod config;
pub mod pipeline;
pub mod pool;
pub mod processor;
pub mod resize;

use exif::Field;
use image::DynamicImage;

pub struct LennaImage {
    pub name: String,
    pub image: Box<DynamicImage>,
    pub path: String,
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
