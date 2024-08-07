#![cfg(feature = "manifest-hashes")]

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

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(always)]
fn sha256_hash(input: impl AsRef<[u8]>) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();

    hasher.update(input);

    format!("{:x}", hasher.finalize())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sha256 hash script", |b| {
        b.iter(|| sha256_hash(black_box(SCRIPT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
