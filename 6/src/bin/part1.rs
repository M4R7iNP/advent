use std::{io::Read, ops::Sub};

/*
struct Pos(isize, isize);

impl std::ops::Add<Pos> for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Self::Output {
        Self(self.0 + other.0, self.1, other.1)
    }
}
*/

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut board = vec![];
    file.read_to_end(&mut board).unwrap();
    let mut board = board
        .split(|c| *c as char == '\n')
        .map(|line| line.to_vec())
        .collect::<Vec<Vec<u8>>>();
    let row_len = board[0].len();
    let mut pos: (isize, isize) = (0, 0);
    let directions: Vec<(isize, isize)> = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut curr_direction_idx = 2;
    'a: for y in 0..board.len() {
        for x in 0..row_len {
            if board[y][x] == '^' as u8 {
                pos = (x as isize, y as isize);
                break 'a;
            }
        }
    }
    let mut steps = 1;
    let mut visited_positions = vec![pos];
    loop {
        let (x, y) = pos;
        println!("{x}x{y}!");
        let char = board[y as usize][x as usize];
        // let char = char as char;
        let mut direction = directions[curr_direction_idx];
        let mut next_pos = (pos.0 + direction.0, pos.1 + direction.1);
        let Some(next_char) = board
            .get(next_pos.1 as usize)
            .and_then(|row| row.get(next_pos.0 as usize))
        else {
            break;
        };
        let next_char = *next_char as char;
        if next_char == '#' {
            curr_direction_idx = (curr_direction_idx + 1) % directions.len();
            direction = directions[curr_direction_idx];
            next_pos = (pos.0 + direction.0, pos.1 + direction.1);
            println!("turn!");
        } else {
        }
        pos = next_pos;
        steps += 1;
        if !visited_positions.contains(&pos) {
            visited_positions.push(pos);
        }
    }
    println!("Answer: {}", visited_positions.len());
}
