/// Returns a vector of strings, split by more than one newline
/// 
/// # Arguments
/// 
/// * `s` - A string to split
/// 
/// # Examples
/// 
/// ```
/// use aoc::utils::split_by_big_gap;
/// assert_eq!(split_by_big_gap("1"), vec!["1"]);
/// 
/// assert_eq!(split_by_big_gap("1\n2\n3"), vec!["1\n2\n3"]);
/// 
/// assert_eq!(split_by_big_gap("1\n    \n2\n   \n3"), vec!["1", "2", "3"]);
/// 
/// assert_eq!(split_by_big_gap("1\n    \n2\n \n3"), vec!["1", "2", "3"]);
/// 
/// assert_eq!(split_by_big_gap("1\n2\n \n3"), vec!["1\n2", "3"]);
/// ```
pub fn split_by_big_gap(s: &str) -> Vec<&str> {
    // This is a bit slow, but it's fine for now
    let mut in_word = true;
    let mut v = Vec::new();
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut current_idx = 0;
    for c in s.chars() {
        if c == '\n' {
            in_word = !in_word;
            if in_word {
                v.push(&s[start_idx..end_idx]);
                start_idx = current_idx + 1;
                end_idx = current_idx + 1;
            } 
        }
        else if !c.is_ascii_whitespace() {
            // We were in newlines, increment double
            if !in_word {
                end_idx += 1;
            }
            in_word = true;
            end_idx += 1;
        }
        current_idx += 1;
    }
    v.push(&s[start_idx..end_idx]);
    v
}