use std::fs;
fn get_data_vec() -> Vec<(u64, u64)> {
    fs::read_to_string("src/input.txt")
        .expect("Error reading file")
        .trim()
        .split(',')
        .map(|pair| {
            let (first, second) = pair.split_once('-').expect("Invalid format");

            (
                first.parse().expect("First num is invalid"),
                second.parse().expect("Second num is invalid")
            )
        })
        .collect()
}

pub fn part1() -> u64 {
    get_data_vec()
        .iter()
        .flat_map(|&(first, second)| first..=second)
        .filter(|&num| check_symmetry_p1(num))
        .sum()
}

fn check_symmetry_p1(num: u64) -> bool {
    let str_of_num = num.to_string();

    if str_of_num.len() % 2 == 1 {
        return false;
    }

    let (first, second) = str_of_num.split_at(str_of_num.len() / 2);

    first == second
}

pub fn part2() -> u64 {
    get_data_vec()
        .iter()
        .flat_map(|&(first, second)| first..=second)
        .filter(|&num| check_symmetry_p2(num))
        .sum()
}

fn check_symmetry_p2(num: u64) -> bool {
    let str_of_num = num.to_string();

    'outer: for i in 1..=str_of_num.len() / 2 {
        if str_of_num.len() % i != 0 {
            continue;
        }

        let (compare, mut rest) = str_of_num.split_at(i);
        for j in 1..str_of_num.len() / i {
            let (new_compare, rest_inner) = rest.split_at(i);

            if compare != new_compare {
                continue 'outer;
            }

            rest = rest_inner;
        }

        return true;
    }

   false
}