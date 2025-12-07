use std::fs::read as read_file;

const SEARCH_CHARS: [char; 3] = ['M', 'A', 'S'];

const DIRECTIONS: [Direction; 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

type XY = (usize, usize);
type Direction = (isize, isize);

struct XmasIterator {
    // pub board: Vec<&'a str>,
    pub board: Vec<Vec<char>>,
    pub debug_board: Vec<Vec<u8>>,
    pub row_len: usize,
    cursor: XY,
}

impl XmasIterator {
    pub fn get_char(&self, x: usize, y: usize) -> Option<char> {
        // if x >= row_len {
        //     return None;
        // }
        self.board
            .get(y)
            .and_then(|row| row.get(x).map(|c| *c as char))
    }

    pub fn search(&mut self, x: usize, y: usize) -> Option<usize> {
        let mut words = 0;
        if self.get_char(x, y)? != 'A' {
            return None;
        }
        for direction in DIRECTIONS {
            let x1 = x as isize - direction.1;
            let y1 = y as isize - direction.0;
            if x1 < 0 || y1 < 0 {
                continue;
            }
            let n = self.continue_direction(
                x1.try_into().unwrap(),
                y1.try_into().unwrap(),
                direction,
                0,
            );
            if n == 100 {
                words += 1;
            }
        }

        if words >= 2 {
            return Some(words);
        }
        return None;
    }
    pub fn continue_direction(
        &mut self,
        x: usize,
        y: usize,
        direction: Direction,
        n: usize,
    ) -> usize {
        // println!("direction: {:?}", direction);
        let sc = self.get_char(x, y);
        if sc.is_some_and(|char| char == SEARCH_CHARS[n]) {
            if n == SEARCH_CHARS.len() - 1 {
                self.debug_board[y][x] = b'S';
                return 100;
            }

            let dx = (x as isize + direction.1) as usize;
            let dy = (y as isize + direction.0) as usize;
            let awd = self.continue_direction(dx, dy, direction, n + 1);
            // self.debug_board[y][x] = 'O';
            if awd >= 3 {
                self.debug_board[y][x] = sc.unwrap().clone() as u8;
            }
            return awd;
        }

        return 0;
    }
}

impl Iterator for XmasIterator {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (cx, cy) = self.cursor;
        let mut x = cx;
        let mut y = cy;
        loop {
            if x >= self.row_len {
                x = 0;
                y += 1;
            }
            if y >= self.board.len() {
                break;
            }

            if let Some(n) = self.search(x, y) {
                self.cursor = (x + 1, y);
                return Some((x, y, n));
            }

            x += 1;
        }

        return None;
    }
}

fn main() {
    let start = std::time::Instant::now();
    let input = String::from_utf8(read_file("input.txt").unwrap()).unwrap();
    let board: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let row_len = board[0].len();
    // let direction_values: Vec<isize> = vec![-row_len, 1, row_len, -1];
    let board_len = board.len();

    let mut successes = 0;
    let mut failures = 0;

    let debug_board = board
        .iter()
        .map(|line| ".".repeat(line.len()).as_bytes().to_owned())
        .collect();
    let mut iter = XmasIterator {
        debug_board,
        board,
        row_len,
        cursor: (0, 0),
    };

    // for y in 0..board_len {
    // for x in 0..row_len {
    // for (x, y, n) in iter {
    while let Some((x, y, n)) = iter.next() {
        // println!("{}x{} = {:?}", x, y, n);
        if n > 0 {
            successes += 1;
        } else {
            failures += 1;
        }
    }
    // }
    // }

    println!("Took: {}ms", start.elapsed().as_millis());
    println!("successes: {}", successes);
    println!("failures: {}", failures);
    println!("=== DEBUG ===");
    for row in &iter.debug_board {
        for char in row {
            print!("{}", *char as char);
        }
        println!("");
    }
}
