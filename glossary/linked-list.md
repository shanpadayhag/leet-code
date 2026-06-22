# Linked List

**In one line:** a chain of little boxes where each box holds one value and a pointer
to the next box, so the data is strung together instead of sitting in one block.

## Plain explanation
Think of a treasure hunt. Each clue gives you one piece of treasure *and* tells you
where to find the next clue. You can't skip ahead to clue #5 — you have to follow the
chain from the start, hop by hop, until you get there.

A linked list works the same way. Each link in the chain is called a **node**, and a
node holds two things:
- a **value** (the actual data — here, a single digit), and
- a **pointer** to the next node (the "where the next clue is" part).

The last node points at *nothing*, which is how you know you've reached the end.

```
┌────┬───┐   ┌────┬───┐   ┌────┬───┐
│ 2  │ ●─┼──>│ 4  │ ●─┼──>│ 3  │ ∅ │
└────┴───┘   └────┴───┘   └────┴───┘
 value next   value next   value next
```

## Why you care
Compare it to its everyday cousin, the **array** (a numbered row of slots, all packed
together). The two are good at *opposite* things, and that trade-off is the whole
reason to know both:

- **Jumping to the 50th item.** An array wins: the slots are packed side by side, so
  "item 50" is a single instant calculation of where that slot sits. A linked list
  loses: there are no slot numbers, so you must follow the chain from the start, 50
  hops, to reach it. (That's O(n) vs O(1) — see [Big-O notation](big-o-notation.md).)
- **Adding or removing at the front/middle.** The linked list wins: you just re-point a
  pointer or two, a fixed bit of work, no matter how long the chain. An array has to
  *shift everything over* to make room, which can be the whole array's worth of work.

So a linked list trades "instant jump to any position" for "cheap stitching in and out
anywhere." For this problem the list is just the input format you're handed, but its
strength fits perfectly: you only ever walk forward one node at a time and tack new
nodes onto the end — exactly the moves linked lists make cheap.

## Quick examples
- **Walking it:** start at the first node (the "head"), read its value, follow its
  pointer to the next, and repeat until a node points at nothing.
- **Building one:** keep a pointer to the *last* node (the "tail"); to add a value,
  create a new node and point the old tail at it — one step, done.
- **Reverse-order digits:** this problem stores `342` as `2 -> 4 -> 3`, so the first
  node you meet is the ones digit — handy for adding right-to-left.

## In code
How each language spells the pieces of this concept:
- Rust: a node is a [`struct`](../languages/rust.md#box) holding a value and a
  [`Option<Box<ListNode>>`](../languages/rust.md#box) — "the next node, or nothing."
  The [`Box`](../languages/rust.md#box) is the pointer that puts the next node on the
  heap; the [`Option`](../languages/rust.md#if-let) is how Rust says "or nothing"
  instead of using a null pointer.

## Related
- [Big-O notation](big-o-notation.md)

## Shows up in
- [2. Add Two Numbers](../problems/0002-add-two-numbers/README.md)
