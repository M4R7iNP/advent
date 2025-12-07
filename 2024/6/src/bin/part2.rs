use std::io::Read;

fn main() {
    let start = std::time::Instant::now();
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut board = vec![];
    file.read_to_end(&mut board).unwrap();
    let mut board = board
        .split(|c| *c == '\n' as u8)
        .map(|line| line.to_vec())
        .collect::<Vec<Vec<u8>>>();
    let row_len = board[0].len();
    let mut pos: (isize, isize) = (0, 0);
    let directions: Vec<(isize, isize)> = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut curr_direction_idx = 2;
    println!("{:?}", board[board.len()-1]);
    if board[board.len()-1].is_empty() {
        board.pop();
    }
    'a: for y in 0..board.len() {
        for x in 0..row_len {
            if board[y][x] == '^' as u8 {
                pos = (x as isize, y as isize);
                break 'a;
            }
        }
    }
    let starting_pos = pos.clone();
    let mut visited_positions = vec![pos];
    loop {
        let (x, y) = pos;
        // println!("{x}x{y}!");
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
            // println!("turn!");
        } else {
        }
        pos = next_pos;
        if !visited_positions.contains(&pos) {
            visited_positions.push(pos);
        }
    }

    visited_positions.sort_unstable();
    let mut answer = 0;
    for new_obstruction in visited_positions {
    // for new_obs_y in 0..board.len() {
    // for new_obs_x in 0..row_len {
    // let new_obstruction = (new_obs_x as isize, new_obs_y as isize);
        if new_obstruction == starting_pos {
            continue;
        }
        let mut pos: (isize, isize) = starting_pos.clone();
        let mut curr_direction_idx = 2;
        let mut board = board.clone();
        // println!("New obstruction: ({}x{})", new_obstruction.0, new_obstruction.1);
        board[new_obstruction.1 as usize][new_obstruction.0 as usize] = '#' as u8;
        let mut visited_path_step_streak = 0;
        let mut visited_pos_streak = vec![pos];
        let mut visited_positions = vec![pos];
        loop {
            let (x, y) = pos;
            // println!("{x}x{y}!");
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
            let next_char = *next_char;
            if next_char == '#' as u8 {
                curr_direction_idx = (curr_direction_idx + 1) % directions.len();
                direction = directions[curr_direction_idx];
                // next_pos = (pos.0 + direction.0, pos.1 + direction.1);
                continue;
            }
            pos = next_pos;
            if visited_pos_streak.contains(&pos) {
                answer += 1;
                break;
            } else if visited_positions.contains(&pos) {
                visited_pos_streak.push(pos);
            } else {
                visited_positions.push(pos);
                visited_pos_streak = vec![];
            }
        }
    // }
    }
    println!("Answer: {answer}");
    println!("Took: {}ms", start.elapsed().as_millis());
}
