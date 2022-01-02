use criterion::{criterion_group, criterion_main, Criterion};
use lenna_core::core::config::Config;
use lenna_core::core::pipeline::Pipeline;
use lenna_core::core::pool::Pool;
use lenna_core::core::LennaImage;
use lenna_core::io::read::read_from_file;
use serde_json::json;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read file", |b| {
        b.iter(|| {
            let image = read_from_file("lenna.png".into()).unwrap();
            assert_eq!(image.exif.len(), 0);
        })
    });

    c.bench_function("run pipeline", |b| {
        b.iter(|| {
            let pipeline = Pipeline::default();
            let mut image = Box::new(LennaImage::default());
            pipeline.run(&mut image).unwrap();
        })
    });

    c.bench_function("run resize pipeline", |b| {
        b.iter(|| {
            let mut config = Config::default();
            let values = json!({
                "width": 400,
                "height": 400
            });
            config.add("resize".to_string(), values);
            let pool = Pool::default();
            let pipeline = Pipeline::new(config, pool);
            let mut image = Box::new(LennaImage::default());
            pipeline.run(&mut image).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
