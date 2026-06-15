# Hash Map

**In one line:** a container that stores **key → value** pairs and can find any
key almost instantly, no matter how many it holds.

## Plain explanation
Think of a coat check at a theater. You hand over your coat and get a numbered
ticket. Later you show the ticket and the attendant walks *straight* to your coat
— they don't search every hook one by one. A hash map works the same way: you give
it a **key** (the ticket), and it jumps directly to the matching **value** (the
coat).

The magic is a "hash function": it turns your key into a location to look, so the
map goes right to the spot instead of scanning everything. That's why looking
something up doesn't get slower as the map grows.

(You'll see it under different names: `HashMap` in Rust, `dict` in Python, "object"
or `Map` in JavaScript, "hash table" in textbooks. Same idea.)

## Why you care
Whenever you catch yourself searching a list over and over for "have I seen this
before?" or "what's paired with this?", a hash map replaces that whole inner
search with one instant lookup. That single swap is what turns a slow `O(n²)`
nested loop into a fast `O(n)` one pass (see [Big-O notation](big-o-notation.md)).

## Quick examples
- **Counting:** tally how many times each word appears → `word → count`.
- **Remembering positions:** store each number's index as you walk a list →
  `value → index` (exactly what Two Sum does).
- **Instant membership:** "is `42` in my collection?" — ask the map, get an
  immediate yes/no instead of scanning.

A mental picture of the speed: finding a name in a 1,000-page phone book by
flipping page-by-page is the slow way; a hash map is like knowing the exact page
the instant you hear the name.

> One caveat: lookups are *average* O(1), not guaranteed. In rare unlucky cases
> many keys land in the same spot and it slows down — but for everyday use you can
> treat it as instant.

## Related
- [Big-O notation](big-o-notation.md)

## Shows up in
- [1. Two Sum](../problems/0001-two-sum/README.md)
