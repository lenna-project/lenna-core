use criterion::{criterion_group, criterion_main, Criterion};
use lenna_core::io::read::read_from_file;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read file", |b| b.iter(|| {
        let image = read_from_file("lenna.png".into()).unwrap();
        assert_eq!(image.exif.len(), 0);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);