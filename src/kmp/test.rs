use super::*;

#[test]
fn assert_test_cases() {
    for (e, t, p) in test_cases() {
        assert_eq!(e, find_first(t.as_bytes(), p.as_bytes()));
    }
}

// TODO: Write benchmarks when they will be stable

fn test_cases() -> Vec<(Option<usize>, &'static str, &'static str)> {
    vec![
        (Some(0), "abcd", "abcd"),
        (Some(1), "abcd", "bcd"),
        (Some(10), "abc abcde abcdef", "abcdef"),
        (None, "abcd", "dcba"),
    ]
}
