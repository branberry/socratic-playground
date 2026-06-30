use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use rust_rag_learn::TextChunker;

fn main() -> Result<()> {
    let data_dir = parse_data_dir()?;
    ingest(&data_dir)
}

fn parse_data_dir() -> Result<PathBuf> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("ingest") => Ok(PathBuf::from("data/sample_docs")),
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
    let chunker = TextChunker::default();
    let mut chunks = Vec::new();

    for entry in std::fs::read_dir(data_dir)
        .with_context(|| format!("reading directory {}", data_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("txt") {
            continue;
        }
        chunks.extend(chunker.chunk_file(&path)?);
    }

    anyhow::ensure!(
        !chunks.is_empty(),
        "no .txt documents found in {}",
        data_dir.display()
    );

    println!("Loaded {} chunk(s) from {}", chunks.len(), data_dir.display());
    for chunk in chunks {
        println!("--- {} ({} chars)", chunk.id, chunk.text.len());
        println!("{}", chunk.text);
    }

    Ok(())
}
