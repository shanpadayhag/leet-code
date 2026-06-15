# Rust syntax handbook

A growing reference of Rust syntax, built from the solutions in this repo. Each
entry is explained once here and linked from wherever it's used. Assumes you know
how to program in *some* language — just not Rust yet.

## Contents
- [`use` declarations](#use)
- [`impl` blocks](#impl)
- [`pub fn` — functions](#pub-fn)
- [`Vec<T>` — growable arrays](#vec-type)
- [`vec![]` — the vector macro](#vec-macro)
- [`let` and `let mut`](#let-mut)
- [`HashMap<K, V>`](#hashmap)
- [`for` + `.iter()` + `.enumerate()`](#for-iter-enumerate)
- [`&` in patterns — destructuring a reference](#ref-pattern)
- [`if let` with `Option` / `Some`](#if-let)
- [`as` — numeric casts](#as-cast)

## `use` declarations {#use}

**In one line:** pulls a name into scope so you can write `HashMap` instead of the
full `std::collections::HashMap` every time.

Rust's standard library is organized into modules with `::` as the separator
(like folders). `use std::collections::HashMap;` says "for the rest of this file,
`HashMap` means that one." Without it the code still works — you'd just have to
type the whole path at each use. Put `use` lines at the top of the file.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `impl` blocks {#impl}

**In one line:** the place where you attach functions to a type.

`impl Solution { ... }` means "here are functions that belong to the `Solution`
type." LeetCode's Rust judge defines an empty `Solution` struct for you and calls
your method on it, which is why the harness code is wrapped in `impl Solution`.
For everyday Rust, `impl` is how you give a struct its methods.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `pub fn` — functions {#pub-fn}

**In one line:** `fn` declares a function; `pub` makes it visible outside its
module.

`pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>` reads as: a public
function named `two_sum`, taking a `Vec<i32>` and an `i32`, returning a `Vec<i32>`.
Parameter types come *after* the name with a colon; the return type follows `->`.
A function with no `->` returns nothing (the unit type `()`). The last expression
in the body — with no semicolon — is the return value, so explicit `return` is
only needed for early exits.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `Vec<T>` — growable arrays {#vec-type}

**In one line:** a resizable list of values that all share one type `T`.

`Vec<i32>` is a vector of 32-bit signed integers. It's Rust's everyday "array that
can grow", living on the heap. The `<i32>` is a *type parameter* — `Vec<String>`,
`Vec<bool>`, etc. all work the same way. You index with `v[0]` and read its length
with `v.len()`.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `vec![]` — the vector macro {#vec-macro}

**In one line:** a shorthand for building a `Vec` from listed values.

`vec![earlier_index, current_index]` creates a two-element vector; `vec![]` creates
an empty one. The `!` marks it as a *macro* (code that expands into more code at
compile time), not a function — that's how it can take any number of arguments and
figure out the element type from them.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `let` and `let mut` {#let-mut}

**In one line:** `let` declares a variable; by default it can't be reassigned, and
`mut` is what makes it changeable.

In Rust, variables are *immutable* unless you opt out. `let x = 5;` binds `x` once
and forever. `let mut x = 5;` lets you later do `x = 6;` or call methods that
mutate it. This default-immutable rule is a safety feature: the compiler stops you
from changing things you didn't mean to. We use `let mut index_of_seen_value`
because we insert into the map as we scan.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `HashMap<K, V>` {#hashmap}

**In one line:** Rust's key → value store, where keys are type `K` and values type
`V`.

This is Rust's spelling of the [hash map](../glossary/hash-map.md) concept.
`HashMap<i32, i32>` maps integer keys to integer values. Create one with
`HashMap::new()` (needs `use std::collections::HashMap;` first). Key methods:
`.insert(key, value)` to store, and `.get(&key)` to look up — note `.get` takes a
*reference* to the key and returns an [`Option`](#if-let) (`Some(value)` if present,
`None` if not), because the key might be missing.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `for` + `.iter()` + `.enumerate()` {#for-iter-enumerate}

**In one line:** loop over a collection, getting both each item and its position.

`for (i, &v) in numbers.iter().enumerate()` breaks down as:
- `.iter()` produces a sequence of *references* to the elements (it borrows the
  vector rather than consuming it, so `numbers` is still usable afterward).
- `.enumerate()` wraps each item into a `(index, item)` pair, counting from 0.
- `for (i, v) in ...` destructures that pair into two names each loop.

The result: `i` walks `0, 1, 2, ...` while `v` walks the values. (For why it's
`&v` and not `v`, see [`&` in patterns](#ref-pattern).)

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `&` in patterns — destructuring a reference {#ref-pattern}

**In one line:** an `&` on the *left* of `=` unwraps a reference, giving you the
value by copy instead of a pointer to it.

`.iter()` yields `&i32` (references to integers), but arithmetic like
`target - current_value` is cleaner on a plain `i32`. Writing the loop variable as
`&current_value` says "the thing I'm receiving is a reference — peel it off and
bind the value underneath." Since `i32` is cheaply copyable, this just copies the
number out. The same move appears in `if let Some(&earlier_index) = ...`, peeling
the reference that `.get()` hands back. Without the `&`, you'd be holding a `&i32`
and would have to dereference with `*` everywhere you used it.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `if let` with `Option` / `Some` {#if-let}

**In one line:** run a block only when a value is present, and pull that value out
in the same line.

Rust has no `null`. A value that might be absent has type `Option<T>`, which is
either `Some(value)` or `None`. `.get()` on a map returns one of these.
`if let Some(&earlier_index) = index_of_seen_value.get(&needed_value) { ... }` means
"if the lookup returned `Some`, bind what's inside to `earlier_index` and run the
block; otherwise skip it." It's the concise alternative to a full `match` when you
only care about one case.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `as` — numeric casts {#as-cast}

**In one line:** converts a number from one type to another, explicitly.

Rust never silently mixes number types, so `current_index` (a `usize`, the type
used for indexes and lengths) must be converted before it can sit in a `Vec<i32>`.
`current_index as i32` performs that conversion. `as` is the blunt cast for
primitives — quick and direct, though for conversions that could lose data Rust
also offers safer checked options elsewhere.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)
