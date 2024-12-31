use criterion::{criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use mole::{
    cli::Args,
    file_explorer::{self, CargoFiles},
};
use std::{hint::black_box, path::PathBuf};

fn search(args: &Args) -> Result<HashMap<String, CargoFiles>, mole::error::MoleError> {
    file_explorer::explore(&args)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search", |b| {
        b.iter(|| {
            search(black_box(&Args {
                name: "serde".to_string(),
                path: PathBuf::from("/Users/pausala/.cargo"),
                deep: true,
                threaded: true,
            }))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
