# Rust warm-up — tutor guide

Your Rust is rusty. Good — this module rebuilds the muscles you'll need for RAG, in order.

**File:** [`src/rust_warmup.rs`](../src/rust_warmup.rs)

**Run tests:**
```bash
cargo test rust_warmup
```

Every exercise starts with `todo!()` — **all tests will fail** until you implement them. That's correct. Fix one exercise at a time.

---

## How we'll work

Same contract as RAG steps:

1. Read the concept comment above each function
2. Try the implementation yourself
3. Run the single test: `cargo test exercise_4` (replace number as needed)
4. Ask for hints only when stuck — tell me what you tried first
5. I won't paste full solutions unless you're truly blocked

**Recommended order:** 1 → 2 → 3 → 4 → 7 → 5 → 6

Exercise 7 reuses 2 and 4 — do those first. Exercises 5–6 are general Rust you'll need later.

---

## Exercise 1 — Borrowing (`byte_len`)

**Concept:** `&str` is a borrowed string slice. Functions take `&str` so callers keep ownership.

**Why it matters for RAG:** `chunk_text(&self, source: &str, text: &str)` borrows both strings — no clone unless you need an owned `String` (like in `Chunk.text`).

**Your task:** Return the length without allocating.

**Verify:** `cargo test exercise_1`

---

## Exercise 2 — Slicing (`first_n_bytes`)

**Concept:** `&text[a..b]` borrows a substring. Indices are **byte** positions, not Unicode code points.

**Why it matters for RAG:** Window chunking slices `text[start..end]`. Our sample docs are ASCII; for production you'd chunk on char boundaries or tokens.

**Your task:** Return `&text[..n]`, or the full string if `n` is too large.

**Verify:**
```bash
cargo test exercise_2
```

**Edge cases to think about:** What if `n` is 0? What if `text` is empty? What if `n` exactly equals `text.len()` (not just greater)? Run `exercise_2_first_n_bytes_edge_cases` — your implementation should pass all of them.

**Discussion:** What would happen if you sliced `"héllo"` at byte index 1? (UTF-8 — why we stick to ASCII in these exercises.)

## Exercise 3 — Iterators (`dot_product`)

**Concept:** Rust prefers iterator chains over index loops.

```text
a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
```

**Why it matters for RAG:** Cosine similarity *is* a dot product (after normalization). You'll write this again in `embed.rs`.

**Your task:** Zip, multiply, sum. Empty slices → `0.0`.

**Verify:** `cargo test exercise_3`

**Discussion:** What happens if `a` and `b` have different lengths? Should you panic, return an error, or zip to the shorter length?

---

## Exercise 4 — Window starts (`window_starts`)

**Concept:** Sliding windows need a list of start indices before you slice.

**Why it matters for RAG:** This is the algorithmic core of Step 1 chunking — separate "where do windows start?" from "build Chunk structs."

**Your task:** Loop `start` from 0, push each value, add `step`, stop when `start >= len`.

**Verify:** `cargo test exercise_4`

**Discussion:** With `len=26, window=10, step=8`, why is the last start `24` and not `16`?

---

## Exercise 7 — Split windows (`split_windows`)

**Concept:** Combine slicing + start indices into a working chunker.

**Why it matters for RAG:** Once this passes, `chunk_text` is mostly wrapping strings in `Chunk { id, source, text }`.

**Your task:**
1. Call `window_starts(text.len(), window, step)`
2. For each start, slice up to `min(start + window, text.len())`
3. Collect owned `String`s (`.to_string()` on each slice)

**Verify:** `cargo test exercise_7`

**Then:** Go implement `chunk_text` in `chunk.rs` — you already know the loop.

---

## Exercise 5 — Option (`first_word`)

**Concept:** `Option<T>` = maybe a value. `None` instead of null.

**Why it matters for RAG:** Parsing API JSON fields, optional metadata, empty queries.

**Your task:** `trim()`, `split_whitespace().next()`.

**Verify:** `cargo test exercise_5`

---

## Exercise 6 — Result (`parse_dimension`)

**Concept:** `Result<T, E>` = success or failure. `?` propagates errors up the call stack.

**Why it matters for RAG:** Every I/O and HTTP call in the pipeline returns `Result`. Same pattern as `ChunkError`.

**Your task:**
- Empty after trim → `Err(ParseError::Empty)`
- `s.parse::<usize>()` — use `.map_err(|_| ParseError::NotANumber)`

**Verify:** `cargo test exercise_6`

---

## After the warm-up

| Warm-up exercise | RAG step it unlocks |
|------------------|---------------------|
| 1–2 Slicing | Step 1 chunking |
| 3 Dot product | Step 2 cosine similarity |
| 4–7 Window loop | Step 1 chunking |
| 5–6 Option/Result | Steps 2–5 error handling |

When all seven pass:

```bash
cargo test rust_warmup   # all green
cargo test chunk         # tackle Step 1 with confidence
```

---

## Cheat sheet (reference, not homework)

| Syntax | Meaning |
|--------|---------|
| `&str` | Borrowed string slice |
| `String` | Owned, growable string |
| `&[f32]` | Borrowed slice of floats |
| `Vec<T>` | Owned growable array |
| `Option<T>` | `Some(value)` or `None` |
| `Result<T, E>` | `Ok(value)` or `Err(error)` |
| `foo?` | Return early on `Err` / `None` (in compatible functions) |
| `.iter()` | Borrow each element |
| `.map(f)` | Transform each element |
| `.collect()` | Build a `Vec` or other collection |
