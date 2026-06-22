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
- [`Box<T>` — a pointer to the heap](#box)
- [`&mut` — mutable references](#mut-ref)
- [`while` loops](#while)
- [`Option::is_some`](#is-some)
- [`Option::take`](#option-take)
- [`Option::as_mut`](#option-as-mut)
- [`.unwrap()`](#unwrap)

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

## `Box<T>` — a pointer to the heap {#box}

**In one line:** a pointer that owns one value stored on the heap, used when a type
needs to point at "more of itself."

**What it is.** Most values live *inline* — right where they're declared. `Box<T>`
instead puts the `T` on the heap (the program's big pool of long-lived memory) and
keeps just a pointer to it. The `Box` owns that value: when the `Box` goes away, the
heap value is freed automatically.

**Why a [linked list](../glossary/linked-list.md) can't live without it.** A node
holds the *next* node. Picture writing it without a box:
```rust
struct ListNode { val: i32, next: Option<ListNode> }   // ❌ does not compile
```
To lay this out, the compiler must know a node's size. But a node contains a node,
which contains a node... forever — an infinitely large type. The error literally says
*"recursive type has infinite size."* A `Box` breaks the cycle: a pointer is a fixed,
known size no matter how big the thing it points to, so the next node lives elsewhere
on the heap and the node's size is finally knowable:
```rust
struct ListNode { val: i32, next: Option<Box<ListNode>> }   // ✅
```

**Trace the types.** `Box::new(ListNode::new(0))` allocates a `ListNode` on the heap
and hands back a `Box<ListNode>` that owns it. You rarely write `*` to reach inside —
field access like `node.val` and `result_tail.next` *auto-dereferences* through the
box for you.

**Why this way.** It's the smallest possible indirection that makes a self-referential
type have a finite size, while still owning its contents (no manual free, no garbage
collector). Pairing it with [`Option`](#if-let) — `Option<Box<ListNode>>` — gives the
two halves of a linked list: "a pointer to the next node" *or* "nothing, this is the
end," with no null pointers involved.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `&mut` — mutable references {#mut-ref}

**In one line:** a borrow you're allowed to *change* the value through — the
read-write counterpart to the read-only [`&`](#ref-pattern).

**The two kinds of borrow.** Two Sum used `&`, a *shared* reference: you can look but
not touch. `&mut` is an *exclusive* reference: you can modify the value in place,
without taking ownership of it. The rule Rust enforces: at any moment a value can have
**many** `&` readers **or exactly one** `&mut` writer — never both at once. That
single-writer guarantee is what makes data races impossible.

**Here.** `let mut result_tail = &mut result_head;` borrows the head node mutably, so
we can grow the list *through* the borrow — `result_tail.next = Some(...)` writes into
the real node, not a copy. Later `result_tail = result_tail.next.as_mut().unwrap();`
re-points the borrow at the freshly added node so the next write lands at the new end.

**Without it.** With a plain `&result_head` the line `result_tail.next = Some(...)`
won't compile — you can't assign through a read-only borrow. The alternative would be
to pass *ownership* of the list around and hand it back each step, which is far
clumsier than borrowing it once and writing through the borrow.

**One confusing overlap.** `mut` shows up in two different roles. In `let mut x`, the
`mut` makes the *binding* reassignable (`x = ...` later). In `&mut x`, the `mut` makes
a *reference through which you can mutate* the pointed-to value. `let mut result_tail =
&mut result_head;` uses both: the binding is reassignable (we re-point it each loop)
*and* it's a mutable reference (we write through it).

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `while` loops {#while}

**In one line:** repeat a block as long as a condition stays true.

`while condition { ... }` checks `condition` before each pass and stops the moment it's
false. Two Sum used a `for` loop because it walked a known sequence to its end. Here we
use `while` because we stop on a *dynamic* condition — "both lists are used up **and**
no carry is left" — not a fixed number of steps:
```rust
while first_digit.is_some() || second_digit.is_some() || carry != 0 { ... }
```

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::is_some` {#is-some}

**In one line:** asks an [`Option`](#if-let) "are you holding a value?" and answers
`true` or `false`.

`first_digit.is_some()` is `true` when `first_digit` is `Some(...)` and `false` when
it's `None`. It only *peeks* — it doesn't take the value out — which is exactly what a
loop condition wants: we check whether digits remain without disturbing them. (Its
mirror image is `.is_none()`.)

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::take` {#option-take}

**In one line:** rips the value out of an `Option`, leaves `None` in its place, and
hands you what was there.

**The problem it solves.** `first_digit` is an owned `Option<Box<ListNode>>`, and we
want the node inside to read its digit and step to `.next`. The obvious
`if let Some(node) = first_digit` *moves* `first_digit` into the match — and Rust then
considers `first_digit` used-up for the rest of the loop, so the next iteration's
`first_digit.is_some()` won't compile. We need the inside *without* destroying the
variable.

**What `.take()` does.** It swaps the slot to `None` and returns the old contents,
working through a `&mut`:
- before: `first_digit` is `Some(box)`
- `first_digit.take()` returns `Some(box)` **and** sets `first_digit` to `None`
- we match the returned `Some(node)`, then immediately overwrite the now-`None`
  `first_digit` with `node.next`

So the variable is always left in a valid state. If the list was already empty,
`.take()` returns `None`, the `if let` simply doesn't fire, and `first_digit` stays
`None` — precisely the "treat a missing digit as nothing" behavior we want.

**Without it.** You'd reach for `std::mem::replace(&mut first_digit, None)` by hand —
which is exactly what `.take()` is shorthand for.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::as_mut` {#option-as-mut}

**In one line:** turns a `&mut Option<T>` into an `Option<&mut T>` — lets you reach a
mutable pointer to the value *inside* without removing it.

After `result_tail.next = Some(Box::new(...))`, we want to advance the tail to that
brand-new node. `.take()` would be wrong here — it would yank the node back out, the
opposite of what we want. `.as_mut()` instead borrows into the `Option`:
- `result_tail.next` is an `Option<Box<ListNode>>`
- `.as_mut()` gives `Option<&mut Box<ListNode>>` — a mutable peek, value left in place
- [`.unwrap()`](#unwrap) pulls out the `&mut Box<ListNode>` we re-point the tail to

So: `.take()` when you mean to *remove* the value, `.as_mut()` when you mean to *keep
it and borrow it*.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `.unwrap()` {#unwrap}

**In one line:** pulls the value out of a `Some` (or an `Ok`), and crashes the program
if it's `None` instead.

`.unwrap()` is the blunt way to get inside an [`Option`](#if-let): on `Some(x)` it
returns `x`, on `None` it panics. That makes it risky in general — a `None` you didn't
expect takes the whole program down. It's safe *here* only because the line right above
just set `result_tail.next = Some(...)`, so the value is provably present; we use
`.unwrap()` to say "I know this is `Some`." When you *can't* prove that, reach for
[`if let`](#if-let) or a `match`, which handle the `None` case instead of exploding.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)
