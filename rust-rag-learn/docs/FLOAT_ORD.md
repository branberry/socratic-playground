# Why `f32`/`f64` don't implement `Ord` (references for Step 3 sort)

## The short version

`f32` and `f64` implement `PartialOrd` but **not** `Ord`. The blocker is **NaN** ("Not a Number"):

- `NaN != NaN` (violates reflexivity required by `Eq`)
- `NaN < x`, `NaN > x`, `NaN <= x`, `NaN >= x` are all `false` (no total order possible)

Because `Ord` requires a total order (every pair comparable, with reflexivity and
transitivity), and NaN breaks both, the standard library refuses to implement `Ord`
for floats. If it did, generic data structures like `BTreeMap` would silently corrupt
on NaN keys.

## The fix in stable Rust

Use `f32::total_cmp` / `f64::total_cmp` (stable since 1.62). It implements the
IEEE 754 (2008) `totalOrder` predicate, which *does* give a total order over all
bit patterns, including NaNs.

```rust
let mut v: Vec<f32> = vec![3.0, f32::NAN, -1.0, 1.0];
v.sort_by(|a, b| a.total_cmp(&b));
```

Ordering under `total_cmp`:
negative quiet NaN < negative signaling NaN < -inf < negative numbers
< -0 < +0 < positive numbers < +inf < positive signaling NaN < positive quiet NaN.

Note: `total_cmp` does **not** agree with `PartialOrd` on `+0.0 == -0.0`
(`total_cmp` treats them as distinct).

## Alternative: `ordered-float` crate

If you want a *wrapper type* that implements `Ord` (and panics on NaN), use
[`ordered-float`](https://crates.io/crates/ordered-float) with `NotNan` or `OrderedFloat`.
Useful when you need floats as keys in `HashMap`/`BTreeMap`.

## Primary sources

- [`f32` primitive — std docs](https://doc.rust-lang.org/std/primitive.f32.html)
  (search the page for "NAN" — the rationale is stated inline)
- [`f64::total_cmp` — core docs](https://doc.rust-lang.org/core/primitive.f64.html#method.total_cmp)
- [`Ord` trait — std docs](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html)
  (the "Examples of incorrect Ord implementations" section uses `f32::NAN` as the
  canonical counter-example)
- [`PartialOrd` trait — std docs](https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html)
  (shows the `f64::NAN.partial_cmp(&1.0) == None` case)
- [StackOverflow: Why does Rust not implement total ordering via Ord for f64/f32?](https://stackoverflow.com/questions/26489701/why-does-rust-not-implement-total-ordering-via-the-ord-trait-for-f64-and-f32)
  (discusses the `Ord: PartialOrd` subtrait design constraint and IEEE 754 §5.10/§5.11)

## Socratic prompts for self-study

1. Why does `Eq` require `a == a` for all `a`, and what does `f32::NAN == f32::NAN`
   return?
2. Why can't the standard library just "pick a slot" for NaN in the ordering
   and implement `Ord` anyway? (Hint: `Ord` is a subtrait of `PartialOrd` — both
   must agree.)
3. In your `search` function, the scores come from `cosine_similarity` on real
   embeddings. Will NaN ever appear there in practice? What would you *want* to
   happen if it did — panic, sort to the end, or something else?
