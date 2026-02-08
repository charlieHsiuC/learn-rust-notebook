# Review of `q0003_repeated_string_match`

Here is a comprehensive review of your initial draft for the "Repeated String Match" problem.

## 1. The Rustacean Way: Idiomatic Refinements

Your draft implements a manual string matching logic using indices and boolean vectors. While this shows a deep effort to optimize, it reinvents the wheel and misses out on Rust's highly optimized standard library functions.

**Key Improvements:**

- **Use `&str` instead of `String`:** Your function takes ownership of `a` and `b`, but it only needs to read them. Changing the signature to `fn(a: &str, b: &str)` is more idiomatic and flexible.
- **Leverage `std` methods:** Instead of manually checking character slices with loops and `step_by`, use `String::contains`. Rust's standard library uses the Two-Way algorithm (a variant of Boyer-Moore), which is extremely fast (`O(N)`).
- **Simplify Logic:** The problem can be reduced to: "Repeat `a` until it's long enough to hold `b`, check if it contains `b`. If not, add one more `a` and check again."

## 2. Algorithm & Complexity

### Your Draft

- **Algorithm:** Manual pattern matching with a precomputed match table (`is_matched`).
- **Time Complexity:** `O(M * N)`. The construction of `is_matched` iterates `M` times, and inside each iteration, it performs a string slice comparison that can take up to `O(N)`. For inputs where `N, M = 10,000`, this approaches `10^8` operations, which is risky for time limits.
- **Space Complexity:** `O(M)` for the `is_matched` vector.

### Idiomatic Solution

- **Algorithm:** String construction and standard library search.
- **Time Complexity:** `O(N + M)`.
  - Constructing the repeated string takes `O(N + M)`.
  - `contains()` takes `O(N + M)` on average.
- **Space Complexity:** `O(N + M)`. We construct a new string that is roughly the length of `a + b`. Given the constraints ($10^4$), this memory usage (approx. 20KB) is negligible.

## 3. Blind Spot Detection

- **Over-Engineering:** The logic in your draft attempts to align `b` against repeats of `a` manually. This is error-prone (off-by-one errors are likely) and hard to read.
- **Correctness:** Your logic for `min_repeat` calculation inside the loop is complex (`(m-len-remain) / n + 2`). While it might work for tested cases, verifying it mathematically is difficult compared to the simulation approach.
- **Edge Cases:** The simulation approach naturally handles cases where `b` is just slightly longer than `a`, or where `b` wraps around `a` multiple times, without complex index arithmetic.

## 4. The "Why"

The idiomatic version is preferred because:

1. **Readability:** It clearly expresses the intent: "Make `a` big enough, then look for `b`."
2. **Performance:** It reduces the time complexity from Quadratic `O(N*M)` to Linear `O(N+M)`.
3. **Maintainability:** Relying on `str::contains` shifts the burden of correctness and optimization to the Rust standard library maintainers.

## 5. Key Rust Concept

- **`String` vs `&str`:** Use `&str` for function arguments when you just need a view into the string data.
- **`String::with_capacity`:** When building a string in a loop, pre-allocating memory prevents frequent re-allocations, improving performance.
- **`contains`:** A powerful method on string slices that implements efficient substring search algorithms.
