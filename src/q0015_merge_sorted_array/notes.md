# Q0015 - Merge Sorted Array

## 1. The Rustacean Way (Draft Review)

你的 `Draft::merge` 已經選對了核心策略：**從尾端往前填**，利用 `nums1` 尾部的空白區域做 in-place merge。這比「先複製再 merge」或「用額外 buffer」都更貼近本題要求。

但以 Rustacean 角度，仍有幾處可改進：

- **索引型別**：`i32` 搭配 `as usize` 反覆轉型是 C/Java 習慣。Rust 中 `len()`、切片索引都用 `usize`，一開始就把 `m`、`n` 轉成 `usize` 會更自然。
- **寫入位置**：`for k in 0..len` 再用 `len - k - 1` 計算目標索引，意圖不直觀。用 `write` 指標從 `m + n` 遞減，語意就是「下一個要填的位置」。
- **分支結構**：外層 `if i >= 0 && j >= 0` 再內層 `if i >= 0` 的巢狀 `if/else` 可收斂成單一 `while` 條件，減少重複賦值邏輯。
- **借用語意**：`nums2` 全程唯讀，簽名應為 `&Vec<i32>`（或 `&[i32]`），而非 `&mut Vec<i32>`，讓編譯器幫你 enforce 不可變借用。

`Solution` 版本：

```rust
while write > 0 {
    write -= 1;
    if j > 0 && (i == 0 || nums2[j - 1] > nums1[i - 1]) {
        nums1[write] = nums2[j - 1];
        j -= 1;
    } else {
        nums1[write] = nums1[i - 1];
        i -= 1;
    }
}
```

這裡 `i == 0` 代表 `nums1` 有效元素已取完；`j == 0` 時走 `else` 分支取 `nums1`。條件合併後，四種 `(i, j)` 狀態都由同一條路徑處理。

## 2. Algorithm & Complexity

### Pattern

- **Two pointers (backward merge)**
- **In-place write into spare capacity**

### Why it works

`nums1` 長度為 `m + n`，後 `n` 格是預留空位。若從前往後 merge，會覆蓋尚未讀取的 `nums1` 元素；改從**最後一格往前填**，每次寫入的位置要麼是空位、要麼是已讀過可覆蓋的值，因此不會丟失資料。

每一步比較 `nums1[i-1]` 與 `nums2[j-1]`，較大者放到 `write-1`，對應指標左移。等效於 merge sort 的 merge 步驟，只是方向相反。

### Complexity

- **Time:** $O(m + n)$ — 每個元素恰好寫入一次
- **Space:** $O(1)$ — 只用常數個索引變數

這是本題在 `m + n <= 200` 下的最優解。

## 3. Blind Spot Detection (Draft)

### A. 邏輯正確，但可讀性有成本

Draft 的演算法本身是對的，沒有覆蓋 bug。主要風險在**維護**：巢狀分支讓 reviewer 需要 mentally trace `i`、`j`、`k` 三個變數的關係，容易在修改時引入 off-by-one。

### B. `i32` 索引的隱患

雖然本題 `m, n <= 200`，不會觸及 `i32` 上限，但 Rust 慣例是用 `usize` 做索引。混用 `i32` 與 `usize` 會：

- 增加 `as usize` / 與 `0` 比較的噪音
- 在更大規模的題目裡更容易出現 sign/width 相關 bug

### C. 相等元素的處理

當 `nums1[i] == nums2[j]` 時，Draft 走 `else` 取 `nums2`。這與標準 merge 的穩定順序一致（先取 `nums2` 或先取 `nums1` 皆可，只要一致）。`Solution` 在相等時同樣取 `nums2`（因條件是 `>` 而非 `>=`）。

### D. 測試覆蓋不足

原 `solution_works` 只驗證 `2 + 2 = 4`。應至少涵蓋：

- 官方範例 1（一般 merge）
- 範例 2（`n = 0`，`nums2` 為空）
- 範例 3（`m = 0`，`nums1` 無有效元素）

## 4. The Why: Why idiomatic Rust here is better

- **更清晰**：`write` 指標直接表達「下一個寫入位置」，比 `len - k - 1` 少一層心算。
- **更安全**：`usize` 索引 + `nums2: &Vec<i32>` 唯讀借用，型別系統幫你擋掉意外修改。
- **同樣高效**：Draft 與 Solution 都是 $O(m+n)$ / $O(1)$，idiomatic 版本沒有效能損失，只有可讀性與型別安全上的提升。
- **可擴展**：若改成 `&mut [i32]` 簽名，可進一步脫離 `Vec` 耦合，但 LeetCode 模板用 `Vec` 已足夠。

## 5. Key Rust Concept (Keywords)

- backward two-pointer merge
- in-place mutation via `&mut Vec<T>`
- `usize` indexing vs. `i32` loop counters
- immutable borrow for read-only input (`&Vec<T>`)
- utilizing spare capacity to avoid overwrite
