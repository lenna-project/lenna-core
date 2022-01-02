use lenna_core::core::config::Config;
use lenna_core::core::pipeline::Pipeline;
use lenna_core::core::pool::Pool;
use lenna_core::io::read::read_from_file;
use lenna_core::io::write::write_to_file;
use serde_json::json;

fn main() {
    println!("This example loads an image, resizes it in a pipeline and saves it.");
    let mut image = Box::new(read_from_file("lenna.png".into()).unwrap());
    let mut config = Config::default();
    let values = json!({
        "width": 400,
        "height": 400
    });
    config.add("resize".to_string(), values);
    let pool = Pool::default();
    let pipeline = Pipeline::new(config, pool);
    pipeline.run(&mut image).unwrap();

    image.name = "lenna_example_write".to_string();
    write_to_file(&image, image::ImageOutputFormat::Jpeg(90)).unwrap();
}
