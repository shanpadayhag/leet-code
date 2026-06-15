# Big-O Notation

**In one line:** a shorthand for how much *slower* (or *hungrier for memory*) a
method gets as the input grows.

## Plain explanation
Imagine you wrote instructions for handing out flyers, and someone asks "how long
will this take?" The honest answer isn't a number of minutes — it depends on how
many flyers there are. Big-O describes the *shape* of that relationship: if you
double the flyers, does the work double, stay the same, or explode?

It deliberately ignores small details (exact seconds, one-time setup) and focuses
on the part that dominates when things get big. We write it with a capital O and
the input size as `n`, like `O(n)`.

## Why you care
Two methods can both "work" but one finishes instantly on a huge input while the
other hangs. Big-O lets you predict that *before* running anything, and it's the
main yardstick for deciding whether a solution is good enough.

## Quick examples
Picture `n` as the number of items:

- **O(1) — constant.** Always the same work, no matter the size. Grabbing one coat
  from a coat check. Doesn't care if there are 10 coats or a million.
- **O(n) — linear.** Work grows in step with the input. Reading every name on a
  guest list once. Twice the names, twice the time.
- **O(n²) — quadratic.** Work grows with the *square* of the input. Comparing every
  guest with every other guest — a handshake between all pairs. 10 guests = 100
  handshakes; 1,000 guests = 1,000,000. This is the trap a hash map helps you
  escape.
- **O(log n) — logarithmic.** Work barely grows even when the input balloons. Each
  step throws away half of what's left, like finding a word in a dictionary by
  repeatedly splitting it in two.

Rough feel for 10,000 items: `O(n)` ≈ 10,000 steps, while `O(n²)` ≈ 100,000,000
steps. Same problem, wildly different patience required.

The same idea applies to memory ("space complexity"): `O(n)` space means the
storage you use grows with the input; `O(1)` space means you use a fixed amount no
matter what.

## Related
- [Hash Map](hash-map.md)

## Shows up in
- [1. Two Sum](../problems/0001-two-sum/README.md)
