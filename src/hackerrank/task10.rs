// https://www.hackerrank.com/challenges/sock-merchant/problem

fn sock_merchant(ar: &[i32]) -> i32 {

    let mut counts = [0; 101];

    for &sock in ar {

        counts[sock as usize] += 1;

    }

    counts.iter().map(|&count| count / 2).sum()

}

#[test]

fn test_sock_merchant() {

    assert_eq!(

        sock_merchant(&[10, 20, 20, 10, 10, 30, 50, 10, 20]),

        3

    );

    assert_eq!(sock_merchant(&[1, 2, 1, 2, 1, 3, 2]), 2);

    assert_eq!(sock_merchant(&[1, 1, 1, 1]), 2);

}