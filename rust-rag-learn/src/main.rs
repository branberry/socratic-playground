use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use rust_rag_learn::chunk::TextChunker;

fn main() -> Result<()> {
    let data_dir = parse_data_dir()?;
    ingest(&data_dir)
}

fn parse_data_dir() -> Result<PathBuf> {
    let mut args = env::args().skip(1);
    let default = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/sample_docs");
    match args.next().as_deref() {
        None | Some("ingest") => Ok(default),
        Some("--data-dir") => args
            .next()
            .map(PathBuf::from)
            .context("--data-dir requires a path"),
        Some(other) => anyhow::bail!(
            "unknown command: {other}\n\nUsage:\n  cargo run -- ingest\n  cargo run -- --data-dir PATH"
        ),
    }
}

fn ingest(data_dir: &PathBuf) -> Result<()> {
    // TODO(step-1): When chunk.rs is ready, wire it up:
    //
    // use rust_rag_learn::TextChunker;
    // let chunker = TextChunker::default();
    // let mut chunks = Vec::new();
    // for each .txt file: chunks.extend(chunker.chunk_file(&path)?);
    // print each chunk.id and chunk.text

    let mut file_count = 0usize;
    let mut total_bytes = 0usize;

    for entry in std::fs::read_dir(data_dir)
        .with_context(|| format!("reading directory {}", data_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("txt") {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        file_count += 1;
        total_bytes += text.len();

        println!(
            "found {} ({} bytes) — implement chunk.rs to split into passages",
            path.file_name().unwrap_or_default().to_string_lossy(),
            text.len()
        );

        let chunker = TextChunker::default();

        let chunks = chunker.chunk_file(&path).expect("Could not chunk file");

        println!("{:?}", chunks);
    }

    anyhow::ensure!(
        file_count > 0,
        "no .txt documents found in {}",
        data_dir.display()
    );

    println!("\n{file_count} file(s), {total_bytes} bytes total. Step 1: build src/chunk.rs");

    Ok(())
}
