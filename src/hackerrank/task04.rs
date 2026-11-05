// https://www.hackerrank.com/challenges/grading/problem
// https://www.hackerrank.com/challenges/grading/problem
fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next = ((grade / 5) + 1) * 5;
            if next - grade < 3 {
                next
            } else {
                grade
            }
        }
    }).collect()
}

#[test]
fn test_grading_students() {
    assert_eq!(
        grading_students(&[73, 67, 38, 33]),
        vec![75, 67, 40, 33]
    );
}