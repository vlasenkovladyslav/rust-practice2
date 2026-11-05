// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

fn breaking_records(scores: &[i32]) -> (i32, i32) {

    let mut best = scores[0];

    let mut worst = scores[0];

    let mut best_count = 0;

    let mut worst_count = 0;

    for &score in scores.iter().skip(1) {

        if score > best {

            best = score;

            best_count += 1;

        }

        if score < worst {

            worst = score;

            worst_count += 1;

        }

    }

    (best_count, worst_count)

}

#[test]

fn test_breaking_records() {

    assert_eq!(

        breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]),

        (2, 4)

    );

    assert_eq!(breaking_records(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42]), (4, 0));

}