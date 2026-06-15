# 1. Two Sum — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). New features are explained in
the [Rust handbook](../../languages/rust.md); already-known ones are linked there.

## New here
This is the first Rust solution in the repo, so every construct below is new and
now lives in the handbook:
- `use std::collections::HashMap;` — [`use` declaration](../../languages/rust.md#use):
  brings `HashMap` into scope so the full path isn't repeated.
- `impl Solution { ... }` — [`impl` block](../../languages/rust.md#impl): the
  wrapper LeetCode's judge expects; it's where methods attach to a type.
- `pub fn two_sum(...) -> Vec<i32>` — [`pub fn`](../../languages/rust.md#pub-fn):
  a public function with typed parameters and a return type after `->`.
- `Vec<i32>` — [vector type](../../languages/rust.md#vec-type): a growable list of
  integers, used for both the input and the answer.
- `let mut index_of_seen_value` — [`let mut`](../../languages/rust.md#let-mut): a
  mutable binding, needed because we insert into the map while scanning.
- `HashMap<i32, i32>` / `HashMap::new()` —
  [`HashMap`](../../languages/rust.md#hashmap): Rust's spelling of the
  [hash map](../../glossary/hash-map.md) concept.
- `for (current_index, &current_value) in numbers.iter().enumerate()` —
  [`for` + `.iter()` + `.enumerate()`](../../languages/rust.md#for-iter-enumerate):
  loops over values *and* their positions at once.
- `&current_value` and `Some(&earlier_index)` —
  [`&` in patterns](../../languages/rust.md#ref-pattern): peels the reference off
  so we work with plain `i32` values.
- `if let Some(&earlier_index) = ...get(&needed_value)` —
  [`if let` with `Option`](../../languages/rust.md#if-let): runs the block only
  when the lookup actually found something.
- `current_index as i32` — [`as` cast](../../languages/rust.md#as-cast): converts
  the `usize` index into the `i32` the answer vector holds.
- `vec![earlier_index, current_index as i32]` / `vec![]` —
  [`vec![]` macro](../../languages/rust.md#vec-macro): builds the result vector.

## Line by line
```rust
let mut index_of_seen_value: HashMap<i32, i32> = HashMap::new();
```
Start an empty map from each number we've passed to the index where we saw it.

```rust
for (current_index, &current_value) in numbers.iter().enumerate() {
```
Walk the list once. Each loop hands us the position (`current_index`) and the value
(`current_value`), with the `&` peeling the reference so `current_value` is a plain
`i32`.

```rust
    let needed_value = target - current_value;
```
The partner we'd need to hit the target. (No `mut` — it's computed fresh each loop
and never reassigned.)

```rust
    if let Some(&earlier_index) = index_of_seen_value.get(&needed_value) {
        return vec![earlier_index, current_index as i32];
    }
```
Ask the map whether we've already seen that partner. `.get` returns `Some(index)`
if yes — and `if let` both tests for `Some` and unwraps the stored index in one
step, so we can return both positions immediately.

```rust
    index_of_seen_value.insert(current_value, current_index as i32);
```
Not found yet, so file the current value away so a *later* number can find it.

```rust
vec![]
```
Unreachable given the problem's "exactly one answer" guarantee, but Rust needs
every path to return a `Vec<i32>`, so we hand back an empty one.
