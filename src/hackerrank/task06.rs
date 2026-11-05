// https://www.hackerrank.com/challenges/kangaroo/problem

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> bool {

    let distance = x2 - x1;

    let speed = v1 - v2;

    speed > 0 && distance % speed == 0

}

#[test]

fn test_kangaroo() {

    assert_eq!(kangaroo(0, 3, 4, 2), true);

    assert_eq!(kangaroo(0, 2, 5, 3), false);

    assert_eq!(kangaroo(0, 2, 4, 2), false);

}