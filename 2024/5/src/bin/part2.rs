use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Rule {
    first: usize,
    second: usize,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines().peekable();
    println!("rules:");
    let mut count = 0;
    let mut rules: Vec<Rule> = vec![];
    while let Some(Ok(line)) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        count += 1;
        let (first, second) = line.split_once('|').unwrap();
        rules.push(Rule {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
        });
    }
    rules.sort_unstable();
    println!("rules: {count}");
    println!("pages:");
    count = 0;
    let mut sum = 0;
    while let Some(Ok(line)) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let pages = line
            .split(',')
            .map(|page| page.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut is_valid = true;
        let mut fixed_pages = pages.clone();
        // for (i, page) in pages.iter().enumerate() {
        let mut i = 0;
        while i < fixed_pages.len() {
            let prev_pages = &fixed_pages[0..i].to_owned();
            let page = fixed_pages[i];
            let mut page_idx = i;
            for rule in rules.iter() {
                if rule.first < page {
                    continue;
                }
                if rule.first > page {
                    break;
                }
                // if prev_pages.iter().find(|prev_page| rule.second == **prev_page).is_some() {
                if let Some(ri) = prev_pages
                    .iter()
                    .enumerate()
                    .find_map(|(ri, prev_page)| (rule.second == *prev_page).then_some(ri))
                {
                    fixed_pages.remove(page_idx);
                    fixed_pages.insert(ri, page);
                    page_idx = ri;
                    is_valid = false;
                    i = 0; // restart
                }
            }
            /*
            if !is_valid {
                break;
            }
*/
            // prev_pages.push(page);
            i += 1;
        }
        if !is_valid {
            count += 1;
            // find middle page
            let middle_idx = fixed_pages.len() / 2;
            sum += fixed_pages[middle_idx];
        }
    }
    println!("pages: {count}");
    println!("answer: {sum}");
}
