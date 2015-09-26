use super::*;

#[test]
fn assert_find_first_test_cases() {
    for (e, t, p) in find_first_test_cases() {
        assert_eq!(e, find_first(t.as_bytes(), p.as_bytes()));
    }
}

#[test]
fn assert_find_all_test_cases() {
    for (e, t, p) in find_all_test_cases() {
        assert_eq!(e, find_all(t.as_bytes(), p.as_bytes()));
    }
}

// TODO: Write benchmarks when they will be stable

fn find_first_test_cases() -> Vec<(Option<usize>, &'static str, &'static str)> {
    vec![
        (Some(0), "abcd", "abcd"),
        (Some(1), "abcd", "bcd"),
        (Some(10), "abc abcde abcdef", "abcdef"),
        (None, "abcd", "dcba"),
    ]
}

fn find_all_test_cases() -> Vec<(Vec<usize>, &'static str, &'static str)> {
    vec![
        (vec![0, 1, 2], "aaa", "a"),
        (vec![0, 1], "aaa", "aa"),
        (vec![1], "abcd", "bcd"),
        (vec![0, 4, 10], "abc abcde abcdef", "abc"),
        (vec![], "abcd", "bcda"),
    ]
}
