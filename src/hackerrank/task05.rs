// https://www.hackerrank.com/challenges/apple-and-orange/problem

fn count_apples_and_oranges(

    s: i32,

    t: i32,

    a: i32,

    b: i32,

    apples: &[i32],

    oranges: &[i32],

) -> (i32, i32) {

    let apple_count = apples.iter()

        .filter(|&&x| a + x >= s && a + x <= t)

        .count() as i32;

    let orange_count = oranges.iter()

        .filter(|&&x| b + x >= s && b + x <= t)

        .count() as i32;

    (apple_count, orange_count)

}

#[test]

fn test_apple_and_orange() {

    assert_eq!(

        count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]),

        (1, 1)

    );

}