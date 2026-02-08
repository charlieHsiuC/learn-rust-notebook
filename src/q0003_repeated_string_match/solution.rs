pub fn repeated_string_match(a: &str, b: &str) -> i32 {
    // Pre-allocate memory to avoid re-allocations.
    // We know the result string will be at least b.len() and at most b.len() + 2 * a.len().
    let mut s = String::with_capacity(b.len() + a.len() * 2);
    let mut count = 0;

    // Repeat `a` until it is at least as long as `b`.
    while s.len() < b.len() {
        s.push_str(a);
        count += 1;
    }

    // Check if `b` is contained.
    if s.contains(b) {
        return count;
    }

    // Append one more `a` to cover the case where `b` starts towards the end of the current `s`.
    s.push_str(a);
    count += 1;

    if s.contains(b) {
        return count;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::repeated_string_match;

    #[test]
    fn it_works() {
        assert_eq!(repeated_string_match("abcd", "cdab"), 2);
        assert_eq!(repeated_string_match("a", "aa"), 2);
        assert_eq!(repeated_string_match("aa", "a"), 1);
        assert_eq!(repeated_string_match("abc", "wxyz"), -1);
        assert_eq!(repeated_string_match("abc", "cabcabca"), 3);
    }
}