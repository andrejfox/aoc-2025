use std::fs;
fn get_data_vec() -> Vec<(char, u16)> {
    fs::read_to_string("src/input.txt")
        .expect("Error reading file")
        .trim()
        .split("\n")
        .map(|line| {
            let (first, rest) = line.split_at(1);

            (
                first.chars().next().unwrap(),
                rest.parse::<u16>().expect("Error parsing number")
            )
        })
        .collect::<Vec<(char, u16)>>()
}

pub fn part1() -> u16 {
    let mut pointer: u16 = 50;
    let mut count: u16 = 0;

    for (direction, amount) in get_data_vec().iter() {
        match direction {
            'R' => {
                pointer = (pointer + amount) % 100;
            }
            'L' => {
                let temp: i16 = pointer as i16 - (amount % 100) as i16;

                if temp >= 0 {
                    pointer = temp as u16;
                } else {
                    pointer = (100 + temp) as u16;
                }
            }
            _ => panic!("Invalid input char!"),
        }

        if pointer == 0 {
            count += 1;
        }
    }

    count
}

pub fn part2() -> u16 {
    let mut pointer: u16 = 50;
    let mut count: u16 = 0;

    for (direction, amount) in get_data_vec().iter() {
        match direction {
            'R' => {
                count += amount / 100;
                if pointer + amount % 100 > 100 {
                    count += 1
                }
                pointer = (pointer + amount) % 100;
            }
            'L' => {
                count += amount / 100;
                let temp: i16 = pointer as i16 - (amount % 100) as i16;
                if pointer == 0 && temp < 0{
                    count -= 1;
                }

                if temp >= 0 {
                    pointer = temp as u16;
                } else {
                    count += 1;
                    pointer = (100 + temp) as u16;
                }
            }
            _ => panic!("Invalid input char!"),
        }

        if pointer == 0 {
            count += 1;
        }
    }

    count
}