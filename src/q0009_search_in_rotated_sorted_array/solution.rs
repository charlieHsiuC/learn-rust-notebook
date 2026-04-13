struct Draft;
struct Solution;

impl Draft {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = -1;
        let mut right: i32 = nums.len() as i32;

        while right - left > 1 {
            let mid = (left + (right - left) / 2) as usize;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[right as usize - 1] > nums[left as usize + 1] {
                if nums[mid] < target {
                    left = mid as i32;
                } else {
                    right = mid as i32;
                }
            } else {
                if target >= nums[left as usize + 1] {
                    if target > nums[mid] {
                        left = mid as i32
                    } else {
                        right = mid as i32;
                    }
                } else {
                    if target > nums[mid] || target <= nums[right as usize - 1] {
                        left = mid as i32;
                    } else {
                        right = mid as i32;
                    }
                }
            }
        }

        -1
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            let value = nums[mid];

            if value == target {
                return mid as i32;
            }

            // Determine which half is sorted, then shrink to the feasible half.
            if nums[left] <= value {
                if nums[left] <= target && target < value {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if value < target && target <= nums[right - 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution};

    #[test]
    fn draft_works() {
        assert_eq!(Draft::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn solution_finds_target_in_rotated_array() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
    }
}
