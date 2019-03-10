/*
#[macro_use]
extern crate criterion;
extern crate rusty_xkcd;

use criterion::Criterion;
use rusty_xkcd::Comic;

fn get_comic_bench(c: &mut Criterion) {
    c.bench_function("Get Comic", |b| b.iter(|| Comic::get_comic(589)));
}

fn get_latest_comic_bench(c: &mut Criterion) {
    c.bench_function("Get Latest Comic", |b| b.iter(|| Comic::get_latest_comic()));
}

DISABLED UNTIL WORKING
fn get_random_comic_bench(c: &mut Criterion) {
    c.bench_function("Get Random Comic", |b| b.iter(|| Comic::get_random_comic()));
}


criterion_group!(comic_benches, get_comic_bench, get_latest_comic_bench);
criterion_main!(comic_benches);

Cargo.toml
[dev-dependencies]
criterion = "0.2.10"

[[bench]]
name = "comics_bench"
harness = false
*/
