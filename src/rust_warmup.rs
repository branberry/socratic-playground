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
    if n >= text.len() {
        return &text;
    }

    &text[..n]
}

// ---------------------------------------------------------------------------
// Exercise 3 — Iterators
// ---------------------------------------------------------------------------
//
// Concept: `.iter().zip().map().sum()` — you'll use this pattern for dot
// products when computing cosine similarity in Step 2.

/// Dot product of two equal-length slices.
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
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

    let mut result: Vec<usize> = vec![];

    for i in (0..len).step_by(step) {
        result.push(i);
    }

    result
}

// ---------------------------------------------------------------------------
// Exercise 5 — Option
// ---------------------------------------------------------------------------
//
// Concept: `Option` means "maybe a value". `.and_then()`, `.map()`, `?` with
// Options inside Results — you'll see this when parsing API responses.

/// Return the first whitespace-delimited word, or `None` if empty/whitespace.
pub fn first_word(text: &str) -> Option<&str> {
    if text.trim().is_empty() {
        None
    } else {
        let strings: Vec<&str> = text.trim().split_whitespace().collect();
        Some(strings[0])
    }
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
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    trimmed.parse::<usize>().map_err(|_| ParseError::NotANumber)
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
    let mut chunks: Vec<String> = vec![];

    for i in (0..text.len()).step_by(step) {
        if i + window > text.len() {
            chunks.push(text[i..].to_string());
            continue;
        }
        let chunk = text[i..i + window].to_string();
        chunks.push(chunk);
    }

    chunks
}

// ---------------------------------------------------------------------------
// Exercise 8 — Explicit lifetimes (function returning &str)
// ---------------------------------------------------------------------------
//
// Concept: When a function returns a reference, Rust needs to know what memory
// it borrows from. `'a` means "this reference must not outlive the inputs."
//
// You've already written `fn first_n_bytes(text: &str, ...) -> &str` — the
// compiler *elided* the lifetime. Here you write it explicitly.
//
// Try uncommenting `broken_temporary()` in the tests module to see the compiler
// reject a dangling reference.

/// Return the longer of `x` or `y` (by byte length). If equal, return `x`.
pub fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        return x;
    } else {
        return y;
    }
}

// ---------------------------------------------------------------------------
// Exercise 9 — Lifetimes in structs
// ---------------------------------------------------------------------------
//
// Concept: If a struct holds references, it needs a lifetime parameter too.
// This is a *borrowed view* of a chunk — no allocation. Your real `Chunk`
// uses `String` because owned data can outlive the original document buffer.
//
// `borrow_first_window` ties together slicing (Ex 2) and lifetimes (Ex 8).

/// A borrowed passage: points into existing string data, does not own it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedPassage<'a> {
    pub source: &'a str,
    pub text: &'a str,
}

/// Borrow the first `window` bytes of `text` alongside its source name.
pub fn borrow_first_window<'a>(
    source: &'a str,
    text: &'a str,
    window: usize,
) -> BorrowedPassage<'a> {
    todo!("Exercise 9: use first_n_bytes(text, window), return BorrowedPassage {{ source, text }}")
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
        assert_eq!(first_n_bytes("abc", 3), "abc");
    }

    #[test]
    fn exercise_2_first_n_bytes_edge_cases() {
        // n = 0 → empty slice, even when text is non-empty
        assert_eq!(first_n_bytes("abcdef", 0), "");
        assert_eq!(first_n_bytes("x", 0), "");

        // empty string — always empty, regardless of n
        assert_eq!(first_n_bytes("", 0), "");
        assert_eq!(first_n_bytes("", 1), "");
        assert_eq!(first_n_bytes("", 100), "");

        // n exactly equals length (boundary, not past end)
        assert_eq!(first_n_bytes("abc", 3), "abc");
        assert_eq!(first_n_bytes("a", 1), "a");

        // n = 1 on multi-byte ASCII string
        assert_eq!(first_n_bytes("hello", 1), "h");

        // single-character string
        assert_eq!(first_n_bytes("z", 0), "");
        assert_eq!(first_n_bytes("z", 1), "z");
        assert_eq!(first_n_bytes("z", 99), "z");
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

    #[test]
    fn exercise_8_longer() {
        assert_eq!(longer("hi", "hello"), "hello");
        assert_eq!(longer("abc", "ab"), "abc");
        assert_eq!(longer("same", "same"), "same");
        assert_eq!(longer("", "x"), "x");
    }

    #[test]
    fn exercise_8_longer_lifetime_at_call_site() {
        // Both inputs live for the whole test — returned reference is valid.
        let owned = String::from("embedding vector search");
        let slice: &str = "search";
        assert_eq!(longer(&owned, slice), "embedding vector search");
    }

    #[test]
    fn exercise_9_borrow_first_window() {
        let doc = "abcdefghijklmnopqrstuvwxyz";
        let passage = borrow_first_window("rust_basics.txt", doc, 5);
        assert_eq!(
            passage,
            BorrowedPassage {
                source: "rust_basics.txt",
                text: "abcde",
            }
        );
    }

    #[test]
    fn exercise_9_borrow_first_window_short_doc() {
        let doc = "hi";
        let passage = borrow_first_window("doc.txt", doc, 100);
        assert_eq!(passage.text, "hi");
    }

    // -----------------------------------------------------------------------
    // Lifetime trap — uncomment to see the compiler error (E0106 / E0515):
    //
    // fn broken_temporary() -> &str {
    //     let s = String::from("temporary");
    //     &s
    // }
    //
    // `s` is dropped at the end of the function; returning &s would be a
    // dangling pointer. Rust rejects this at compile time.
    // -----------------------------------------------------------------------
}
