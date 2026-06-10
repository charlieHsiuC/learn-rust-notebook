# Q0014 - Reverse Pairs

## 1. The Rustacean Way (Draft Review)

你的 `Draft::reverse_pairs` 有幾個值得肯定的點：

- 有先思考 overflow（你特別處理了 `i32::MAX`）。
- 有使用 iterator chain，程式看起來偏 declarative。

但以 Rustacean 角度，這版仍有明顯可以改進的地方：

- 可讀性：`rev().enumerate()` + `arr[..len - 1 - i]` 的索引轉換非常繞，讀者要花時間確認 `i, j` 的相對關係。
- 意圖不直接：`(*v + 1) >> 1 > *x` 這種位元技巧雖然在數學上可行，但不直觀，維護成本高。
- 型別語意：題目本質是比較 `nums[i] > 2 * nums[j]`，直接升級到 `i64` 比較會比位移 trick 更安全、更語義化。

`Solution` 使用 merge sort + two pointers，讓「演算法意圖」清楚對應題目：

- 分治排序：保證左右半段有序。
- 跨區間統計：對每個左半元素，用右半遞增指標一次掃過。
- 最後 merge 回原切片。

這樣的寫法同時符合 Rust 慣用風格：

- 用 `&mut [i32]` 做遞迴，避免不必要的擁有權搬移。
- 借助 `split_at_mut` 明確劃分可變借用區域。
- 用 `i64` 進行比較，避免 `2 * x` 溢位風險。

## 2. Algorithm & Complexity

### Pattern

- Divide and Conquer (Merge Sort counting)
- Two pointers for cross-half pair counting

### Why it works

在遞迴的某一層，左右兩半都已排序：

- 左半：`nums[l..mid)`
- 右半：`nums[mid..r)`

對每個左半元素 `nums[i]`，在右半找最大的 `j` 使得 `nums[i] > 2 * nums[j]` 仍成立。
因為右半有序，`j` 只會往右移，不會回頭，所以整層 cross-count 是線性的。

### Complexity

- Time: $O(n \log n)$
- Space: $O(n)$（一個共用暫存 buffer）

這是本題在 `n <= 5 * 10^4` 下的標準可接受解。

## 3. Blind Spot Detection (Draft)

### A. 效能瓶頸（最關鍵）

Draft 是雙層掃描，時間複雜度為 $O(n^2)$。

- 最差約 $\frac{n(n-1)}{2}$ 次比較。
- 當 $n = 5 * 10^4$，量級會到 $1.25 * 10^9$，在 LeetCode 通常會 TLE。

### B. 邏輯可讀性風險

`(*v + 1) >> 1 > *x` 雖可避免 `2 * x` overflow，但：

- 很難一眼看出與題目條件等價。
- 團隊協作時不易被快速驗證。

### C. 邊界條件驗證不足

原測試只驗證 `2 + 2 = 4`，沒有覆蓋：

- 官方範例
- 負數混合
- `i32::MIN / i32::MAX` 極值

`Solution` 補了這些測試，降低 regression 風險。

## 4. The Why: Why idiomatic Rust here is better

- 更穩健：`i64` comparison 直覺又安全，不需位元 trick。
- 更可維護：分治 + merge 的結構是經典模板，後續 debug/優化容易。
- 更高效：從 $O(n^2)$ 降到 $O(n \log n)$，在資料上限差異巨大。
- 更符合 Rust 精神：用切片、借用與明確邊界管理可變性，安全且高效。

## 5. Key Rust Concept (Keywords)

- `&mut [T]` slice-based recursion
- `split_at_mut` for disjoint mutable borrows
- `copy_from_slice` for efficient merge write-back
- integer widening (`i32 -> i64`) for overflow-safe arithmetic
- divide-and-conquer pattern in Rust ownership model
