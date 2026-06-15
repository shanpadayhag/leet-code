# 1. Two Sum

| | |
|---|---|
| Difficulty | Easy |
| Languages  | Rust |
| Pattern    | [Hash Map](../../glossary/hash-map.md) |
| Time/Space | O(n) / O(n) |
| Source     | [LeetCode](https://leetcode.com/problems/two-sum/) |

## The Problem
You're given a list of whole numbers and a single **target** number. Somewhere in
the list there are exactly two numbers that add up to the target. Your job is to
return the **positions** (indexes) of those two numbers.

What matters:
- There is always **exactly one** answer, so you never have to worry about "no
  match" or "many matches".
- You can't use the same element twice (the two positions must be different).
- The list can be large (up to ~10,000 numbers), and values can be negative.

Tiny example:
```
numbers = [2, 7, 11, 15], target = 9
answer  = [0, 1]          because numbers[0] + numbers[1] = 2 + 7 = 9
```

## Understand It

### In plain words
Imagine you're at a till with a fistful of coupons, and you need two coupons that
together total exactly $9. You could compare every coupon against every other
coupon... or you could be clever. For each coupon you pick up, you already know
the exact partner you're hoping to find — if this coupon is worth $2, you need a
$7 partner. So instead of comparing everything to everything, you just remember
the coupons you've already seen and check whether the partner you need is among
them.

### The slow, obvious way
Take `numbers = [2, 7, 11, 15]`, `target = 9`. The brute-force idea: try every
possible pair and check if it adds up to 9.

| Pair checked | Sum | Is it 9? |
|---|---|---|
| 2 + 7  | 9  | ✅ yes! |
| (and if it weren't) 2 + 11 | 13 | no |
| 2 + 15 | 17 | no |
| 7 + 11 | 18 | no |
| ... | ... | ... |

We got lucky early here, but in the worst case you compare every number with
every other number. For a list of `n` numbers that's roughly `n × n` checks — on
10,000 numbers, that's 100 million comparisons. Slow. (See
[Big-O notation](../../glossary/big-o-notation.md) for why "`n × n`" is the part
that hurts.)

### The trick
Here's the insight: **you don't need to search for both numbers — only for the
partner of the one you're holding.**

If the current number is `2` and the target is `9`, the partner you need is
`9 - 2 = 7`. So the real question at each step is just: *"Have I already seen a 7?"*

Answering "have I seen this value before?" instantly is exactly what a
[hash map](../../glossary/hash-map.md) is for. We walk the list once, and for each
number we (1) check if its needed partner is already in the map, and (2) if not,
drop the current number into the map so future numbers can find *it*.

### Watch it run
`numbers = [3, 2, 4]`, `target = 6`. The map stores `value → index` of everything
we've passed.

| Step | Current value | Needed partner (target − value) | Partner already in map? | Action | Map after step |
|---|---|---|---|---|---|
| 1 | `3` (index 0) | `6 − 3 = 3` | no | remember `3` | `{3: 0}` |
| 2 | `2` (index 1) | `6 − 2 = 4` | no | remember `2` | `{3: 0, 2: 1}` |
| 3 | `4` (index 2) | `6 − 4 = 2` | **yes — at index 1!** | return `[1, 2]` | — |

At step 3 we needed a `2`, and we'd already filed a `2` away at index 1. Done.

### The answer
The result is `[1, 2]`, because `numbers[1] + numbers[2] = 2 + 4 = 6`. The trick is
guaranteed correct because by the time we hold any number, every number *before*
it is already in the map — so if its partner exists earlier in the list, we will
always find it. And since the problem promises exactly one pair exists, the first
match we hit is *the* answer.

## The Code
### Rust
```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_of_seen_value: HashMap<i32, i32> = HashMap::new();

        for (current_index, &current_value) in numbers.iter().enumerate() {
            let needed_value = target - current_value;
            if let Some(&earlier_index) = index_of_seen_value.get(&needed_value) {
                return vec![earlier_index, current_index as i32];
            }
            index_of_seen_value.insert(current_value, current_index as i32);
        }

        vec![]
    }
}
```

**Time:** O(n) — one pass over the list, and each map lookup/insert is O(1) on
average.   **Space:** O(n) — in the worst case we store almost every number in the
map before finding the pair.

## Remember This
When a problem asks *"is there an element with property X?"* and you're tempted to
write a loop inside a loop, ask whether you can **remember what you've seen** in a
[hash map](../../glossary/hash-map.md) and turn the inner search into an instant
lookup. This "trade memory for speed, look back instead of searching forward"
pattern turns `O(n²)` into `O(n)` and shows up everywhere — pair-sum problems,
finding duplicates, substring problems, and counting.
