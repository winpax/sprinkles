const SCRIPT: &str = r#"$basename = $url.split('/')[-1]
$basenameNoExt = $basename.split('.')[0]
$version = $basenameNoExt.split('_')[-1]

$url = "https://github.com/ScoopInstaller/Main/releases/download/v$version/scoop-windows-x86_64-$version.zip"
$hash = "e2a1c7dd49d547fdfe05fc45f0c9e276cb992bd94af151f0cf7d3e2ecfdc4233"

$basename = $url.split('/')[-1]
$basenameNoExt = $basename.split('.')[0]
$version = $basenameNoExt.split('_')[-1]

$url = "https://github.com/ScoopInstaller/Main/releases/download/v$version/scoop-windows-x86_64-$version.zip"
$hash = "e2a1c7dd49d547fdfe05fc45f0c9e276cb992bd94af151f0cf7d3e2ecfdc4233"

$basename = $url.split('/')[-1]
$ext = $basename.split('.')[-1]

$url = "https://github.com/ScoopInstaller/Main/releases/download/v$version/scoop-windows-x86_64-$version.zip"
$hash = "e2a1c7dd49d547fdfe05fc45f0c9e276cb992bd94af151f0cf7d3e2ecfdc4233"
"#;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

#[inline(always)]
fn sha256_hash(input: impl AsRef<[u8]>) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();

    hasher.update(input);

    format!("{:x}", hasher.finalize())
}

#[inline(always)]
fn blake3_hash(input: impl AsRef<[u8]>) -> String {
    let mut hasher = blake3::Hasher::new();

    hasher.update(input.as_ref());

    format!("{}", hasher.finalize())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sha256 hash script", |b| {
        b.iter(|| sha256_hash(black_box(SCRIPT)))
    });

    c.bench_function("blake3 hash script", |b| {
        b.iter(|| blake3_hash(black_box(SCRIPT)))
    });

    const SMALL_FILE_PATH: &str = "benches/sfsu-x86_64-1.14.0-beta.1.exe";

    c.bench_function("sha256 hash small file", |b| {
        b.iter_batched(
            || std::fs::read(black_box(SMALL_FILE_PATH)).expect("could not read file"),
            |file| sha256_hash(black_box(file)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("blake3 hash small file", |b| {
        b.iter_batched(
            || std::fs::read(black_box(SMALL_FILE_PATH)).expect("could not read file"),
            |file| blake3_hash(black_box(file)),
            BatchSize::SmallInput,
        )
    });

    // const LARGE_FILE_PATH: &str = "benches/sfsu-x86_64-1.14.0-beta.1.exe";

    // c.bench_function("sha256 hash large file", |b| {
    //     b.iter_batched(
    //         || std::fs::read(black_box(SMALL_FILE_PATH)).expect("could not read file"),
    //         |file| sha256_hash(black_box(file)),
    //         BatchSize::SmallInput,
    //     )
    // });

    // c.bench_function("blake3 hash large file", |b| {
    //     b.iter_batched(
    //         || std::fs::read(black_box(SMALL_FILE_PATH)).expect("could not read file"),
    //         |file| blake3_hash(black_box(file)),
    //         BatchSize::SmallInput,
    //     )
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
