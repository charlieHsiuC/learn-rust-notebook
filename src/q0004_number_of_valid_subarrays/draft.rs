pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut stk: Vec<(i32, usize)> = Vec::new();

    for &v in nums[..].iter().rev() {
        let mut curr = 1;
        while let Some(&(min_val, acc)) = stk.last() {
            if v <= min_val {
                stk.pop();
                curr += acc;
            }
            else {
                break;
            }
        }
        stk.push((v, curr));
        count += curr;
    }

    return count as i32;
}

#[cfg(test)]
mod tests {
    use super::valid_subarrays;

    #[test]
    fn it_works() {
        assert_eq!(valid_subarrays(vec![1, 4, 2, 5, 3]), 11);
        assert_eq!(valid_subarrays(vec![2, 2, 2]), 6);
        assert_eq!(valid_subarrays(vec![3, 2, 1]), 3);
        assert_eq!(valid_subarrays(vec![14,20,5,13,18,8,2,14,3,13]), 19);
    }
}