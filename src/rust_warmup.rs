//! Rust warm-up exercises — work through these before (or alongside) RAG steps.
//!
//! Each function has a matching test. Implement the body, run:
//!
//! ```bash
//! cargo test rust_warmup
//! ```
//!
//! Read `docs/RUST_WARMUP.md` for concepts and tutor prompts.

// ---------------------------------------------------------------------------
// Exercise 1 — Borrowing (&str)
// ---------------------------------------------------------------------------
//
// Concept: `&str` is a borrowed view into string data. You can read it without
// owning it. This is how `chunk_text(source: &str, text: &str)` receives input.

/// Return the number of bytes in `text` without taking ownership.
pub fn byte_len(text: &str) -> usize {
    text.len()
}

// ---------------------------------------------------------------------------
// Exercise 2 — Slicing
// ---------------------------------------------------------------------------
//
// Concept: `&text[start..end]` borrows a substring. You'll slice windows in
// `chunk_text`. These tests use ASCII only (1 byte = 1 char).

/// Borrow the first `n` bytes of `text`.
///
/// If `n >= text.len()`, return the full string.
pub fn first_n_bytes(text: &str, n: usize) -> &str {
    if n > text.len() {
        return &text;
    }

    &text[0..n]
}

// ---------------------------------------------------------------------------
// Exercise 3 — Iterators
// ---------------------------------------------------------------------------
//
// Concept: `.iter().zip().map().sum()` — you'll use this pattern for dot
// products when computing cosine similarity in Step 2.

/// Dot product of two equal-length slices.
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    todo!("Exercise 3: zip a and b, multiply pairs, sum")
}

// ---------------------------------------------------------------------------
// Exercise 4 — Building a Vec in a loop
// ---------------------------------------------------------------------------
//
// Concept: Sliding-window chunking needs start indices: 0, step, 2*step, ...
// until the next window would start at or past `len`.
//
// Example: window_starts(26, 10, 8) → [0, 8, 16, 24]
//   (window=10, step=8 because overlap=2)

/// Return start indices for a sliding window over `[0..len)`.
///
/// - `window` — max window width (unused here, but kept for clarity)
/// - `step` — how far to advance each iteration (`chunk_size - chunk_overlap`)
///
/// Stop when the next start index would be >= `len`.
pub fn window_starts(len: usize, window: usize, step: usize) -> Vec<usize> {
    let _ = window;
    todo!("Exercise 4: loop from 0, push start, advance by step, stop when start >= len")
}

// ---------------------------------------------------------------------------
// Exercise 5 — Option
// ---------------------------------------------------------------------------
//
// Concept: `Option` means "maybe a value". `.and_then()`, `.map()`, `?` with
// Options inside Results — you'll see this when parsing API responses.

/// Return the first whitespace-delimited word, or `None` if empty/whitespace.
pub fn first_word(text: &str) -> Option<&str> {
    todo!("Exercise 5: trim, split_whitespace, next")
}

// ---------------------------------------------------------------------------
// Exercise 6 — Result and error mapping
// ---------------------------------------------------------------------------
//
// Concept: Fallible operations return `Result`. `map_err` converts error types —
// same pattern as `ChunkError::Io` in chunk.rs.

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    NotANumber,
}

/// Parse a positive embedding dimension from a string (e.g. `"128"` → `Ok(128)`).
pub fn parse_dimension(s: &str) -> Result<usize, ParseError> {
    todo!("Exercise 6: trim, reject empty, parse with str::parse, map errors")
}

// ---------------------------------------------------------------------------
// Exercise 7 — Putting it together
// ---------------------------------------------------------------------------
//
// Concept: This is chunking without the `Chunk` struct — just strings.
// Once this passes, you have the core loop for `chunk_text`.

/// Split `text` into fixed-width windows (byte indices, ASCII-safe).
///
/// - Each window is at most `window` bytes.
/// - Advance by `step` bytes between windows.
/// - Last window may be shorter.
pub fn split_windows(text: &str, window: usize, step: usize) -> Vec<String> {
    todo!("Exercise 7: use window_starts + first_n_bytes/slicing, collect into Vec<String>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_1_byte_len() {
        assert_eq!(byte_len("hello"), 5);
        assert_eq!(byte_len(""), 0);
    }

    #[test]
    fn exercise_2_first_n_bytes() {
        assert_eq!(first_n_bytes("abcdef", 3), "abc");
        assert_eq!(first_n_bytes("abc", 10), "abc");
    }

    #[test]
    fn exercise_3_dot_product() {
        assert!((dot_product(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]) - 32.0).abs() < 1e-5);
        assert_eq!(dot_product(&[], &[]), 0.0);
    }

    #[test]
    fn exercise_4_window_starts() {
        assert_eq!(window_starts(26, 10, 8), vec![0, 8, 16, 24]);
        assert_eq!(window_starts(5, 10, 8), vec![0]);
        assert_eq!(window_starts(0, 10, 8), vec![]);
    }

    #[test]
    fn exercise_5_first_word() {
        assert_eq!(first_word("  hello world  "), Some("hello"));
        assert_eq!(first_word("   "), None);
    }

    #[test]
    fn exercise_6_parse_dimension() {
        assert_eq!(parse_dimension("128"), Ok(128));
        assert_eq!(parse_dimension("  64  "), Ok(64));
        assert_eq!(parse_dimension(""), Err(ParseError::Empty));
        assert_eq!(parse_dimension("abc"), Err(ParseError::NotANumber));
    }

    #[test]
    fn exercise_7_split_windows() {
        let parts = split_windows("abcdefghijklmnopqrstuvwxyz", 10, 8);
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "abcdefghij");
        assert_eq!(parts[1], "ijklmnopqr");
        assert_eq!(parts[2], "qrstuvwxyz");
        assert_eq!(parts[3], "yz");
        assert!(parts.iter().all(|p| p.len() <= 10));
    }
}
