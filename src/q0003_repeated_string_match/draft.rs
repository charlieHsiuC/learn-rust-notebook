use std::cmp::min;

pub fn repeated_string_match(a: String, b: String) -> i32 {
    let n = a.len();
    let m = b.len();
    let mut min_repeat: usize = m / n + 2;

    if m <= n {
        if a.contains(&b) {
            return 1;
        }
        
        let c = a.clone() + &a;
        if c.contains(&b) {
            return 2;
        }
        
        return -1;
    }

    let is_matched: Vec<bool> = (0..m).map(|idx| {
        let end = min(idx+n, m);
        a[..(end-idx)] == b[idx..end]
    }).collect();

    let is_found = (1..=n)
    .filter(|&len| {
        if is_matched[m-len] {
            if ((0..=m-len-n)).rev().step_by(n).all(|idx| is_matched[idx]) {
                let remain: usize = (m-len) % n;
                if remain == 0 {
                    min_repeat = min(min_repeat, (m-len-remain) / n + 1);
                    return true;
                }
                else if a[n-remain..] == b[..remain] {
                    min_repeat = min(min_repeat, (m-len-remain) / n + 2);
                    return true;
                }
            }
        }
        false
    })
    .any(|_| true); 

    if is_found {
        min_repeat as i32
    }
    else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::repeated_string_match;

    #[test]
    fn it_works() {
        assert_eq!(repeated_string_match("aaaaaaaaab".to_string(), "ba".to_string()), 2);
        assert_eq!(repeated_string_match("abcd".to_string(), "cdab".to_string()), 2);
        assert_eq!(repeated_string_match("aa".to_string(), "a".to_string()), 1);
        assert_eq!(repeated_string_match("a".to_string(), "aa".to_string()), 2);
    }
}