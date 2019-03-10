#[macro_use]
extern crate criterion;
extern crate rusty_xkcd;

use criterion::Criterion;
use rusty_xkcd::*;

fn get_comic_bench(c: &mut Criterion) {
    c.bench_function("Get Comic", |b| b.iter(|| Comic::get_comic(589).unwrap()));
}

fn get_latest_comic_bench(c: &mut Criterion) {
    c.bench_function("Get Comic", |b| b.iter(|| Comic::get_latest_comic().unwrap()));
}

/* DISABLED UNTIL WORKING
fn get_random_comic_bench(c: &mut Criterion) {
    c.bench_function("Get Comic", |b| {
        b.iter(|| Comic::get_random_comic().unwrap())
    });
}
*/

criterion_group!(
    comic_benches,
    get_comic_bench,
    get_latest_comic_bench
);
criterion_main!(comic_benches);
