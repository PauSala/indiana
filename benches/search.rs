use criterion::{criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use mole::{
    file_explorer::{self, CargoFiles},
    parallel_explorer,
};
use std::{hint::black_box, path::PathBuf};

fn _search_series(path: &PathBuf) -> Result<HashMap<String, CargoFiles>, mole::error::MoleError> {
    let mut hash = HashMap::new();
    file_explorer::collect_files(path, &mut hash, true)?;
    Ok(hash)
}

fn search_parallel(path: &PathBuf) -> Result<HashMap<String, CargoFiles>, mole::error::MoleError> {
    let hash = parallel_explorer::collect_files(path, true)?;
    Ok(hash)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search", |b| {
        b.iter(|| search_parallel(black_box(&PathBuf::from("/Users/pausala/.cargo"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
