// https://www.hackerrank.com/challenges/staircase/problem
fn staircase(n: i32) -> String {
    let mut result = String::new();
    for i in 1..=n {
        result.push_str(&" ".repeat((n - i) as usize));
        result.push_str(&"#".repeat(i as usize));
        if i < n {
            result.push('\n');
        }
    }
    result
}
#[test]
fn test_staircase() {
    assert_eq!(staircase(4), "   #\n  ##\n ###\n####");
}
