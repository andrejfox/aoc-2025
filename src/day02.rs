use std::fs;
fn get_data_vec() -> Vec<(u64, u64)> {
    fs::read_to_string("src/input.txt")
        .expect("Error reading file")
        .trim()
        .split(',')
        .map(|pair| {
            let mut nums = pair.split('-');

            let first = nums.next()
                .expect("Failed getting first number")
                .parse::<u64>()
                .expect("First number not a number");

            let second = nums.next()
                .expect("Failed getting second number")
                .parse::<u64>()
                .expect("Second number not a number");

            (first, second)
        })
        .collect()
}

pub(crate) fn part1() -> u64 {
    let data_vec: Vec<(u64, u64)> = get_data_vec();


    let mut count: u64 = 0;
    for (first, second) in data_vec.iter() {
        for i in *first..*second + 1 {
            if check_symmetry_p1(i) {
                count += i;
            }
        }
    }

    count
}

fn check_symmetry_p1(num: u64) -> bool {
    let str_of_num = num.to_string();

    if str_of_num.len() % 2 == 1 {
        return false;
    }

    let (first, second) = str_of_num.split_at(str_of_num.len() / 2);

    first == second
}

pub(crate) fn part2() -> u64 {
    let data_vec: Vec<(u64, u64)> = get_data_vec();


    let mut count: u64 = 0;
    for (first, second) in data_vec.iter() {
        for i in *first..*second + 1 {
            if check_symmetry_p2(i) {
                dbg!(i);
                count += i;
            }
        }
    }

    count
}

fn check_symmetry_p2(num: u64) -> bool {
    let str_of_num = num.to_string();

    'outer: for i in 1..str_of_num.len() / 2 + 1 {
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