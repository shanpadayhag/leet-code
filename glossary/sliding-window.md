# Sliding Window

**In one line:** keep a stretch of a list (a "window") with a left and right edge,
and slide it forward instead of re-checking the same stretch over and over.

## Plain explanation
Picture reading a long sentence through a cardboard slot that shows only a few
words at a time. To find, say, the longest phrase that follows some rule, you don't
lift the slot and start over at every word — you just **widen or shrink the slot as
you glide it rightward**. You push the right edge to take in a new word; when the
words inside break your rule, you pull the left edge inward until they obey again.

The key property: both edges only ever move **forward**. The right edge sweeps
across the whole thing once, and the left edge chases it, also only forward. Even
though the window is constantly resizing, each edge travels the length of the list
just once — so the total work is proportional to the length, not to length times
length.

Two flavours you'll meet:
- **Fixed-size window:** the slot is always `k` wide; you slide it and recompute.
- **Variable-size window** (this problem): the slot grows and shrinks to stay valid,
  and you track the best size (or count) seen.

## Why you care
Whenever a problem says "find the longest / shortest / best **contiguous** run
(substring, subarray) that satisfies X," the brute-force instinct is to try every
start and re-scan forward from each — that's `O(n²)`. A sliding window replaces all
that re-scanning with two forward-only pointers and turns it into one `O(n)` pass
(see [Big-O notation](big-o-notation.md)).

The word that signals it: **contiguous**. If the pieces have to be next to each
other, a window usually fits. (If you're free to pick scattered elements, it usually
doesn't.)

## Quick examples
- **Longest repeat-free run:** widen the window until a character repeats, then pull
  the left edge past the earlier copy — the length of the widest valid window is the
  answer.
- **Smallest run summing to at least `T`:** widen until the sum reaches `T`, then
  shrink from the left while it still does, tracking the smallest width.
- **Fixed window of size `k`:** slide a `k`-wide slot across and, at each step, add
  the entering element and drop the leaving one — no need to re-add the middle.

A window pairs naturally with a [hash map](hash-map.md) when the rule is about
*which* elements are inside (e.g. "no duplicates"): the map answers "is this element
in my window, and where?" instantly, which is what keeps each step cheap.

## Related
- [Hash Map](hash-map.md)
- [Big-O notation](big-o-notation.md)

## Shows up in
- [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/README.md)
