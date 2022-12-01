use crate::utils;

#[test]
fn split_by_big_gap() {
    let input = "1";
    assert_eq!(utils::split_by_big_gap(input), vec!["1"]);
    let input = "1\n2\n3";
    assert_eq!(utils::split_by_big_gap(input), vec!["1\n2\n3"]);
    let input = "1\n    \n2\n   \n3";
    assert_eq!(utils::split_by_big_gap(input), vec!["1", "2", "3"]);
    let input = "1\n    \n2\n \n3";
    assert_eq!(utils::split_by_big_gap(input), vec!["1", "2", "3"]);
    let input = "1\n2\n \n3";
    assert_eq!(utils::split_by_big_gap(input), vec!["1\n2", "3"]);
}