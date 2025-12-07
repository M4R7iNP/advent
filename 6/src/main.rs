use std::{fs::File, io::prelude::*, io::BufReader};

#[derive(Debug)]
struct Race {
    duration: usize,
    distance: usize,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let time_line = lines_iter.next().unwrap().unwrap();
    assert_eq!(time_line[.."Time:".len()], *"Time:");
    let durations = time_line["Time:".len()..]
        .trim()
        .split(char::is_whitespace)
        .filter(|str| !str.is_empty())
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let distance_line = lines_iter.next().unwrap().unwrap();
    assert_eq!(distance_line[.."Distance:".len()], *"Distance:");
    let distances = distance_line["Distance:".len()..]
        .trim()
        .split(char::is_whitespace)
        .filter(|str| !str.is_empty())
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut races: Vec<Race> = vec![];

    for i in 0..durations.len() {
        let duration = durations[i];
        let distance = distances[i];
        races.push(Race { duration, distance });
    }

    println!("Races: {races:?}");

    let mut total_winning_possibilities = 1;
    for race in races {
        let record_distance = race.distance;
        let race_duration = race.duration;

        let mut winning_possibilities: usize = 0;

        for test_button_duration in 0..race_duration {
            let test_distance_traveled = test_button_duration * (race_duration - test_button_duration);
            if test_distance_traveled > record_distance {
                winning_possibilities += 1;
            }
        }
        total_winning_possibilities *= winning_possibilities;
    }

    println!("Total winning possibilities: {total_winning_possibilities:?}");
}
