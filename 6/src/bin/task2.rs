use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let time_line = lines_iter.next().unwrap().unwrap();
    assert_eq!(time_line[.."Time:".len()], *"Time:");
    let race_duration = time_line["Time:".len()..]
        .replace(' ', "")
        .parse::<usize>().unwrap();

    let distance_line = lines_iter.next().unwrap().unwrap();
    assert_eq!(distance_line[.."Distance:".len()], *"Distance:");
    let record_distance = distance_line["Distance:".len()..]
        .replace(' ', "")
        .parse::<usize>().unwrap();

    let mut winning_possibilities: usize = 0;

    for test_button_duration in 0..race_duration {
        let test_distance_traveled = test_button_duration * (race_duration - test_button_duration);
        if test_distance_traveled > record_distance {
            winning_possibilities += 1;
        }
    }

    println!("Winning possibilities: {winning_possibilities:?}");
}
