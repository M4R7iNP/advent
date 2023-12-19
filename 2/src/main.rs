use std::{fs::File, io::Read};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cube_game.pest"]
pub struct CubeGameParser;

#[derive(Debug)]
enum BoxColor {
    Red,
    Green,
    Blue,
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let program = CubeGameParser::parse(Rule::program, &input)
        .unwrap()
        .next()
        .unwrap();
    let games = program.into_inner();
    let mut sum = 0;
    for game in games {
        // println!("{:?}", game);
        let mut game_id: Option<usize> = None;
        let mut valid = true;
        let mut max_red_count = 0;
        let mut max_green_count = 0;
        let mut max_blue_count = 0;
        for pair in game.into_inner() {
            match pair.as_rule() {
                Rule::game_id => {
                    println!("game_id: {:?}", pair);
                    println!("game_id: {:?}", pair.as_str().parse::<usize>());
                    game_id = Some(pair.as_str().parse::<usize>().unwrap());
                }
                Rule::round => {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::box_stmt => {
                                let box_stmt = pair.into_inner();
                                println!("box_stmt: {:?}", box_stmt);
                                let mut color: Option<BoxColor> = None;
                                let mut count: Option<usize> = None;

                                for pair in box_stmt {
                                    match pair.as_rule() {
                                        Rule::box_color => {
                                            color = match pair.as_str() {
                                                "red" => Some(BoxColor::Red),
                                                "green" => Some(BoxColor::Green),
                                                "blue" => Some(BoxColor::Blue),
                                                _ => None,
                                            };
                                            // println!("hei: {:?}", color);
                                        }
                                        Rule::box_count => {
                                            // println!("{:?}", pair.as_str());
                                            count = Some(
                                                pair.as_str().trim().parse::<usize>().unwrap(),
                                            );
                                        }
                                        _ => {}
                                    }
                                }

                                // println!("box: color: {color:?}, count: {count:?}");

                                let count = count.unwrap();
                                match color.unwrap() {
                                    BoxColor::Red => {
                                        red += count;
                                    }
                                    BoxColor::Green => {
                                        green += count;
                                    }
                                    BoxColor::Blue => {
                                        blue += count;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    /*
                    if red > 12 || green > 13 || blue > 14 {
                        valid = false;
                    }
                    */

                    max_red_count = max_red_count.max(red);
                    max_green_count = max_green_count.max(green);
                    max_blue_count = max_blue_count.max(blue);
                }
                _ => {
                    println!("hmm: {:?}", pair);
                }
            }
        }

        println!("valid? {valid:?}");
        /*
        if valid {
            sum += game_id.unwrap();
        }
        */
        sum += max_red_count * max_green_count * max_blue_count;
    }

    println!("sum: {sum}");
}
