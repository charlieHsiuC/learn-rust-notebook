# q0009 - Search in Rotated Sorted Array

## 1) The Rustacean Way

Your `Draft::search` is a good attempt to keep binary-search structure, but it mixes sentinel boundaries (`left = -1`, `right = len`) with frequent `as usize` casts. In Rust, this tends to be brittle.

An idiomatic implementation usually:

- Keeps indices in `usize` only.
- Uses half-open interval `[left, right)` to avoid out-of-bound math.
- Uses clear branch structure to identify which half is sorted.

Refined pattern (already implemented in `Solution::search`):

- `left = 0`, `right = nums.len()`
- `mid = left + (right - left) / 2`
- If left half is sorted (`nums[left] <= nums[mid]`), check whether `target` is inside that range.
- Otherwise, right half is sorted, and do the symmetric check.

## 2) Algorithm & Complexity

Underlying pattern: **Binary Search on a Rotated Monotonic Array**.

- At least one half (`[left, mid]` or `[mid, right)`) is always sorted.
- Each step discards half of the search space.

Complexity:

- Time: $O(\log n)$
- Space: $O(1)$

## 3) Blind Spot Detection

### Potential issues in `Draft::search`

- Empty array panic risk:
  - `nums[right as usize - 1]` is invalid when `len = 0`.
- Fragile sorted-range detection:
  - `nums[right - 1] > nums[left + 1]` is not the right invariant for deciding global monotonicity of the current interval.
- Heavy integer casting (`i32 <-> usize`):
  - Easy to hide boundary bugs.
- Boundary style is hard to reason about:
  - Sentinel `-1` can work mathematically, but in Rust indexing world it increases mental overhead and risk.

### Edge cases to verify

- `[]`, any `target` -> `-1`
- `[x]`, `target == x` and `target != x`
- Two-element rotated arrays like `[3,1]`
- Target at pivot boundary (first/last element)

## 4) Why the idiomatic version is better

- **Safety:** no negative indices, no risky casts for indexing.
- **Correctness readability:** branch logic maps directly to the problem invariant (which half is sorted).
- **Maintainability:** easier for future you to debug or adapt (e.g., duplicate-handling variant).
- **Performance:** same asymptotic complexity, with cleaner control flow and fewer conversions.

## 5) Key Rust Concept

- `usize`-first indexing mindset
- Half-open ranges: `[left, right)`
- Borrowed read-only access with `Vec` indexing
- Invariant-driven branching (express logic with clear conditions)
- Avoiding unnecessary casts for safer boundary handling
