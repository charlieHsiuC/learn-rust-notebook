struct Draft;
struct Solution;

impl Draft {
    fn find_kth_helper(nums: &mut Vec<i32>, l: usize, r: usize, k: usize) -> i32 {
        if l >= r {
            return nums[l];
        }

        let mut j: usize = l + 1;
        let pvt: i32 = nums[l];
        let mut count = 1;
        for i in (l + 1..r) {
            if nums[i] > pvt {
                nums.swap(i, j);
                j += 1;
            } else if nums[i] == pvt {
                count += 1;
            }
        }
        nums.swap(l, j - 1);

        let p_rank = j - l;
        if p_rank <= k && k < p_rank + count {
            return nums[j - 1];
        } else if p_rank > k {
            return Self::find_kth_helper(nums, l, j - 1, k);
        } else {
            return Self::find_kth_helper(nums, j, r, k - p_rank);
        }
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut arr: Vec<i32> = nums.clone();
        Self::find_kth_helper(&mut arr, 0, nums.len(), k as usize)
    }
}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        *nums.select_nth_unstable(n - k as usize).1
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution};

    #[test]
    fn draft_works() {
        assert_eq!(Draft::find_kth_largest(vec![3, 1, 2, 4], 2), 3);
        assert_eq!(Draft::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Draft::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn solution_works() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
