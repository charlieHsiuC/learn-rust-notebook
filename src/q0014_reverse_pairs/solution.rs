struct Draft;
struct Solution;

impl Draft {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let arr: &[i32] = &nums;

        arr.iter()
            .rev()
            .enumerate()
            .map(|(i, x)| {
                //let x2 = 2 * *x;
                arr[..len - 1 - i]
                    .into_iter()
                    .filter(|&v| {
                        if *v == i32::MAX {
                            (*v) >> 1 >= *x
                        } else {
                            (*v + 1) >> 1 > *x
                        }
                    })
                    .count() as i32
            })
            .sum()
    }
}

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        fn sort_count(nums: &mut [i32], buf: &mut [i32]) -> i64 {
            let n = nums.len();
            if n <= 1 {
                return 0;
            }

            let mid = n / 2;
            let mut count = 0_i64;

            {
                let (left, right) = nums.split_at_mut(mid);
                let (buf_left, buf_right) = buf.split_at_mut(mid);
                count += sort_count(left, buf_left);
                count += sort_count(right, buf_right);
            }

            let mut j = mid;
            for i in 0..mid {
                while j < n && (nums[i] as i64) > 2_i64 * (nums[j] as i64) {
                    j += 1;
                }
                count += (j - mid) as i64;
            }

            let (mut i, mut k, mut t) = (0, mid, 0);
            while i < mid && k < n {
                if nums[i] <= nums[k] {
                    buf[t] = nums[i];
                    i += 1;
                } else {
                    buf[t] = nums[k];
                    k += 1;
                }
                t += 1;
            }

            while i < mid {
                buf[t] = nums[i];
                i += 1;
                t += 1;
            }

            while k < n {
                buf[t] = nums[k];
                k += 1;
                t += 1;
            }

            nums.copy_from_slice(&buf[..n]);
            count
        }

        let mut nums = nums;
        let mut buf = vec![0; nums.len()];
        sort_count(&mut nums, &mut buf) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution};

    #[test]
    fn draft_works() {
        assert_eq!(Draft::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    }

    #[test]
    fn solution_examples_work() {
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    }

    #[test]
    fn solution_handles_negative_and_extreme_values() {
        assert_eq!(Solution::reverse_pairs(vec![-5, -5]), 1);
        assert_eq!(Solution::reverse_pairs(vec![i32::MAX, i32::MIN]), 1);
        assert_eq!(Solution::reverse_pairs(vec![i32::MIN, i32::MAX]), 0);
    }
}
