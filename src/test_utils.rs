use crate::utils;

#[test]
fn split_by_big_gap() {
    assert_eq!(utils::split_by_big_gap("1"), vec!["1"]);
    assert_eq!(utils::split_by_big_gap("1\n2\n3"), vec!["1\n2\n3"]);
    assert_eq!(utils::split_by_big_gap("1\n    \n2\n   \n3"), vec!["1", "2", "3"]);
    assert_eq!(utils::split_by_big_gap("1\n    \n2\n \n3"), vec!["1", "2", "3"]);
    assert_eq!(utils::split_by_big_gap("1\n2\n \n3"), vec!["1\n2", "3"]);
}