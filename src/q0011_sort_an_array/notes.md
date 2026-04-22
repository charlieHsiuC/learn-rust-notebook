# Q0011 Sort an Array — Review Notes

## 1. The Rustacean Way

The draft is functionally correct but carries several non-idiomatic patterns inherited from C/C++ style.

**Problems in the draft:**

- `sort_helper` takes `&mut Vec<i32>` with explicit `l`/`r` usize bounds — a classic C-style sentinel approach. In Rust, the idiomatic equivalent is `&mut [i32]` slices, which encode the range directly in the type.
- Index arithmetic like `merged[i - l]` is error-prone noise that vanishes with slices.
- `Vec::<i32>::from(nums)` is verbose — `nums` is already a `Vec<i32>`, just bind it with `mut`.
- Using `<` (strict less-than) for comparison makes the sort *unstable*. Using `<=` restores stability as a habit.

**Improved merge sort (idiomatic):**

```rust
fn merge_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len < 2 { return; }
    let mid = len / 2;
    merge_sort(&mut nums[..mid]);
    merge_sort(&mut nums[mid..]);

    // Only clone the left half — right half stays in nums
    let left = nums[..mid].to_vec();
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < len - mid {
        if left[i] <= nums[mid + j] {
            nums[k] = left[i]; i += 1;
        } else {
            nums[k] = nums[mid + j]; j += 1;
        }
        k += 1;
    }
    // Copy any remaining left elements
    nums[k..k + (left.len() - i)].copy_from_slice(&left[i..]);
    // Right tail is already in place
}
```

Key gains:

- Works on `&mut [i32]` — no Vec-specific API leakage into the helper.
- Only clones the left half (O(n/2) per merge instead of O(n)).
- Uses `copy_from_slice` for a bulk remainder copy rather than a hand-written loop.

---

## 2. Algorithm & Complexity

### Draft — Merge Sort

| | |
|---|---|
| Time | O(n log n) — T(n) = 2T(n/2) + O(n), solved by Master Theorem |
| Aux Space | O(n) — a new `Vec` is allocated per merge level |
| Stack | O(log n) recursion depth |
| Pattern | Divide & Conquer |

### Solution — Heap Sort (in-place)

| | |
|---|---|
| Time | O(n log n) — O(n) to build the heap + n × O(log n) extractions |
| Aux Space | **O(1)** — all operations on the original array |
| Stack | O(1) — uses an iterative sift-down loop |
| Pattern | In-place Priority Queue / Selection |

Heap sort is the answer to "smallest space complexity possible" for an O(n log n) sort. It never allocates and its sift-down loop eliminates even the O(log n) recursion stack.

---

## 3. Blind Spot Detection

1. **Stability:** Using strict `<` makes the sort unstable (equal elements may swap). The problem doesn't require stability for `i32`, but `<=` is the safer habit.
2. **usize underflow:** `r - l` in the draft is always safe here because `r >= l` is invariant, but mixing `usize` subtraction is a landmine. The slice-based approach eliminates this entirely — the compiler enforces it.
3. **Empty input:** `sort_helper(nums, 0, 0)` → `r - l == 0 < 2` → returns. ✓
4. **Single element:** Same guard catches it. ✓
5. **Heap sort is not stable:** Elements with equal values may reorder. Acceptable here since the problem only asks for ascending order, not sort stability.
6. **Per-level allocation:** The draft allocates a fresh `Vec` on every merge call. For a large array, this means O(n log n) allocator calls. The improved version allocates once per merge but only half-sized.

---

## 4. The Why

| Aspect | `&mut Vec<i32>` + bounds | `&mut [i32]` slice |
|---|---|---|
| Type safety | Bounds are runtime values | Range is part of the type |
| API surface | Can accidentally call `.push()` / `.truncate()` | Slice API is minimal and correct |
| Composability | Tied to `Vec` | Works with any contiguous memory (`Box<[T]>`, arrays, etc.) |
| Runtime cost | Identical | Identical — zero overhead |

Slices are Rust's lingua franca for "a view into contiguous memory." Passing `&mut [T]` to recursive helpers is the idiomatic contract: the callee can read and write the elements but cannot change the length. It's both more expressive and safer.

Heap sort is preferred when space matters: it operates entirely in-place, trading the elegance of merge sort for guaranteed O(1) auxiliary memory. The iterative sift-down avoids even the O(log n) call stack.

---

## 5. Key Rust Concepts

- `&mut [T]` — mutable slice as a function argument (vs `&mut Vec<T>`)
- `split_at` / `split_at_mut` — zero-copy slice splitting
- `to_vec()` — clone a slice into an owned `Vec`
- `copy_from_slice` — bulk copy between equal-length slices of `Copy` types
- `slice::swap` — in-place element swap with borrow-checker approval
- In-place vs. allocating algorithms and their space trade-offs
- Iterative vs. recursive algorithms and stack depth considerations
