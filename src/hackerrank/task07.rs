// https://www.hackerrank.com/challenges/between-two-sets/problem

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {

    let mut count = 0;

    for x in 1..=100 {

        if a.iter().all(|&n| x % n == 0)

            && b.iter().all(|&n| n % x == 0)

        {

            count += 1;

        }

    }

    count

}

#[test]

fn test_between_two_sets() {

    assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);

    assert_eq!(get_total_x(&[2, 6], &[24, 36]), 2);

}