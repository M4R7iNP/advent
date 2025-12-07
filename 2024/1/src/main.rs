const INPUT: &str = include_str!("input.txt");

fn first_task() {
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];
    for line in INPUT.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|str| str.parse::<usize>().expect("Not a number"))
            .collect::<Vec<usize>>();
        b.push(nums.pop().expect("Missing number"));
        a.push(nums.pop().expect("Missing number"));
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    let mut sum = 0;

    loop {
        let Some(a_num) = a_iter.next() else {
            break;
        };
        let Some(b_num) = b_iter.next() else {
            break;
        };
        let dist = (b_num as isize - a_num as isize).abs() as usize;
        sum += dist;
    }

    println!("Result: {sum}");
}

fn second_task() {
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];
    for line in INPUT.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|str| str.parse::<usize>().expect("Not a number"))
            .collect::<Vec<usize>>();
        b.push(nums.pop().expect("Missing number"));
        a.push(nums.pop().expect("Missing number"));
    }

    a.sort_unstable();
    b.sort_unstable();

    let ref mut a_iter = a.into_iter().peekable();
    let ref mut b_iter = b.into_iter().peekable();

    let mut sum = 0;

    loop {
        let Some(ref a_num) = a_iter.next() else {
            break;
        };
        let mut count_of_a_nums = 1;
        while let Some(ref aa_num) = a_iter.next_if(|a_num2| a_num2 <= a_num) {
            if aa_num == a_num {
                count_of_a_nums += 1;
            }
        }
        let mut occurrences_in_b = 0;
        while let Some(ref b_num) = b_iter.next_if(|b_num| b_num <= a_num) {
            if b_num == a_num {
                occurrences_in_b += 1;
            }
        }
        let similarity_score = a_num * occurrences_in_b * count_of_a_nums;
        // println!("{similarity_score} = {a_num} * {occurrences_in_b} * {count_of_a_nums}");
        sum += similarity_score;
    }

    println!("Result: {sum}");
}

fn main() {
    first_task();
    second_task();
}
