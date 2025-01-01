use criterion::{criterion_group, criterion_main, Criterion};
use dotenv::dotenv;
use hashbrown::HashMap;
use mole::{
    cli::Args,
    file_explorer::{self, CargoFiles},
};
use std::env;
use std::{hint::black_box, path::PathBuf};

fn search(args: &Args) -> Result<HashMap<String, CargoFiles>, mole::error::MoleError> {
    file_explorer::explore(&args)
}

fn criterion_benchmark(c: &mut Criterion) {
    dotenv().ok();
    let path = PathBuf::from(env::var("DEFAULT_BENCHES_FOLDER").unwrap());
    c.bench_function("search", |b| {
        b.iter(|| {
            search(black_box(&Args {
                name: "serde".to_string(),
                path: path.clone(),
                deep: true,
                threaded: true,
                filter: None,
            }))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
