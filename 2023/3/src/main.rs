use std::io::prelude::*;
use std::ops::RangeInclusive;
use std::{fs::File, io::BufReader};

type Pos = (usize, usize);

fn parse_number(digits_str: &str) -> usize {
    let number = digits_str
        .chars()
        .enumerate()
        .map(|(idx, n)| -> usize {
            let n = n.to_digit(10).unwrap() as usize;
            n * 10_usize.pow((digits_str.len() - idx - 1) as u32)
        })
        .sum();

    return number;
}

struct NumberFinder<'a> {
    cursor: usize,
    line: &'a str,
}

impl<'a> NumberFinder<'a> {
    fn new(line: &'a str) -> Self {
        Self { cursor: 0, line }
    }
}

impl<'a> Iterator for NumberFinder<'a> {
    // x, number
    type Item = (RangeInclusive<usize>, usize);

    // https://github.com/varnishcache/varnish-cache/blob/a3bc025c2df28e4a76e10c2c41217c9864e9963b/lib/libvcc/vmodtool.py#L890
    fn next(&mut self) -> Option<Self::Item> {
        let mut start = 0;
        let mut inside = false;
        let slice = &self.line[self.cursor..];

        let old_cursor = self.cursor;
        for (i, c) in slice.char_indices() {
            self.cursor += 1;
            if inside {
                if !c.is_numeric() {
                    let digits_str = &slice[start..i];
                    let range = RangeInclusive::new(old_cursor + start, old_cursor + i - 1);
                    let number = parse_number(digits_str);
                    return Some((range, number));
                }
            } else {
                if c.is_numeric() {
                    start = i;
                    inside = true;
                }
            }
        }

        if inside {
            let digits_str = &slice[start..];
            let range = RangeInclusive::new(old_cursor + start, self.cursor - 1);
            let number = parse_number(digits_str);
            return Some((range, number));
        }

        None
    }
}

#[derive(Debug)]
struct SearchForSymbolResult {
    char: char,
    char_pos: Pos,
}

fn search_for_symbol(
    lines: &Vec<Option<&String>>,
    digits_range: RangeInclusive<usize>,
) -> Option<SearchForSymbolResult> {
    let mut searched_chars = vec![];

    // search for a symbol in every column that surrounds the number
    let mut search: Vec<(usize, usize)> = Vec::new();
    let adjecent_line_start = (*digits_range.start() as isize - 1).max(0) as usize;
    let adjecent_line_end = *digits_range.end() + 1;
    let adjecent_line_x_range = RangeInclusive::new(adjecent_line_start, adjecent_line_end);
    search.append(
        &mut adjecent_line_x_range
            .clone()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| (*x, 0))
            .collect(),
    );
    search.push((adjecent_line_start, 1));
    search.push((adjecent_line_end, 1));
    search.append(
        &mut adjecent_line_x_range
            .clone()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| (*x, 2))
            .collect(),
    );

    for search in &search {
        let (sx, sy) = search;
        let Some(sline) = lines[*sy] else {
            continue;
        };

        let Some(schar) = sline.chars().nth(*sx) else {
            continue;
        };

        if matches!(
            schar,
            '#' | '$' | '%' | '&' | '*' | '+' | '-' | '/' | '=' | '@'
        ) {
            return Some(SearchForSymbolResult {
                char: schar,
                char_pos: (*sx, *sy),
            });
        } else {
            searched_chars.push(schar);
        }
    }

    return None;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Star {
    coords: Pos,
    number: usize,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut prev_line: Option<String> = None;
    let mut line_iter = reader.lines().peekable();
    let mut sum = 0;
    let mut stars: Vec<Star> = vec![];
    let mut y = 0;
    while let Some(line) = line_iter.next() {
        let line = line.unwrap();
        let number_iter = NumberFinder::new(line.as_str());
        let next_line = line_iter.peek().and_then(|res| res.as_ref().ok());
        let buf_lines: Vec<Option<&String>> = vec![prev_line.as_ref(), Some(&line), next_line];
        // let y = 1isize;

        for (range, number) in number_iter {
            let found = search_for_symbol(&buf_lines, range);
            if let Some(result) = found {
                sum += number;
                if result.char == '*' {
                    stars.push(Star {
                        number,
                        coords: (
                            result.char_pos.0,
                            result.char_pos.1 + y - 1, // off by one, since 1 is current line
                        ),
                    });
                }
            }
        }

        prev_line = Some(line);
        y += 1;
    }

    let mut stars_sum = 0;
    stars.sort_by(|a, b| a.coords.cmp(&b.coords));
    let mut stars_iter = stars.into_iter().peekable();
    while let Some(star) = &stars_iter.next() {
        if let Some(next_star) = stars_iter.peek() {
            if star.coords == next_star.coords {
                let next_star = stars_iter.next().unwrap(); // advance
                stars_sum += star.number * next_star.number;
                continue;
            }
        }
    }
    println!("Sum: {sum}");
    println!("Stars sum: {stars_sum}");
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use crate::{search_for_symbol, NumberFinder};

    #[test]
    fn test_number_finder() {
        let result = NumberFinder::new("123..1.2.3....123").collect::<Vec<_>>();
        assert_eq!(
            result,
            vec![
                (0..=2, 123),
                (5..=5, 1),
                (7..=7, 2),
                (9..=9, 3),
                (14..=16, 123),
            ]
        );
    }

    fn test_for_symbol(lines: Vec<String>, range: RangeInclusive<usize>, expected: bool) {
        let lines: Vec<_> = lines.iter().map(|str| Some(str)).collect();
        assert_eq!(search_for_symbol(&lines, range).is_some(), expected);
    }

    #[test]
    fn none() {
        test_for_symbol(
            vec![
                ".....".into(), //
                ".123.".into(), //
                ".....".into(), //
            ],
            1..=3,
            false,
        );
    }

    #[test]
    fn upper_left() {
        test_for_symbol(
            vec![
                "$....".into(), //
                ".123.".into(), //
                ".....".into(), //
            ],
            1..=3,
            true,
        );
    }

    #[test]
    fn upper_right() {
        test_for_symbol(
            vec![
                "....@".into(), //
                ".123.".into(), //
                ".....".into(), //
            ],
            1..=3,
            true,
        );
    }

    #[test]
    fn bottom_right() {
        test_for_symbol(
            vec![
                ".....".into(), //
                ".123.".into(), //
                "....#".into(), //
            ],
            1..=3,
            true,
        );
    }
}
