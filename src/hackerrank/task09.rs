// https://www.hackerrank.com/challenges/migratory-birds/problem

fn migratory_birds(arr: &[i32]) -> i32 {

    let mut counts = [0; 6];

    for &bird in arr {

        counts[bird as usize] += 1;

    }

    let mut result = 1;

    for bird in 2..=5 {

        if counts[bird] > counts[result] {

            result = bird;

        }

    }

    result as i32

}

#[test]

fn test_migratory_birds() {

    assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);

    assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);

    assert_eq!(migratory_birds(&[1, 2, 2, 1]), 1);

}