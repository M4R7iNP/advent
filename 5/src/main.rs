use std::io::prelude::*;
use std::ops::{Range, RangeBounds, RangeInclusive};
use std::{fs::File, io::BufReader};
use btree_range_map::RangeMap;

// 226172555 first corrrect
// 2463399554 wrong high
// 6060804 wrong low
// 687082519 wrong high
// 400263298 not right
// 687082519
// 400263298
// 74639702 wrong
// 400263298
// 74639702
// 400263298

#[derive(Debug)]
struct MappingEntry {
    source: usize,
    length: usize,
    destination: usize,
}

impl MappingEntry {
    fn source_range(&self) -> RangeInclusive<usize> {
        self.source..=(self.source + self.length)
    }
    fn destination_range(&self) -> RangeInclusive<usize> {
        self.destination..=(self.destination + self.length)
    }
    fn get_destination(&self, val: &usize) -> Option<usize> {
        if self.source_range().contains(val) {
            // return Some(self.source + self.length + val - self.destination - self.length);
            return Some(val + self.destination - self.source);
        }
        None
    }
}

trait Mapping {
    fn get_by_mapping(&self, val: &usize) -> usize;
}

impl Mapping for Vec<MappingEntry> {
    fn get_by_mapping(&self, val: &usize) -> usize {
        for entry in self {
            if entry.source_range().contains(val) {
                return entry.source - val + entry.destination;
            }
        }
        return val.clone();
    }
}

#[derive(Debug)]
struct SeedMapping {
    seed: usize,
    len: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

impl SeedMapping {
    fn new(seed: usize, len: usize) -> SeedMapping {
        SeedMapping {
            seed,
            len,
            soil: seed,
            fertilizer: seed,
            water: seed,
            light: seed,
            temperature: seed,
            humidity: seed,
            location: seed,
        }
    }
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    let seeds: Vec<usize>;
    let mut seed_mapping: Vec<SeedMapping>;

    let mut lines_iter = reader.lines().map_while(|line| line.ok());

    // parse seeds
    {
        let line = lines_iter.next().unwrap();
        assert_eq!(line[0.."seeds: ".len()], *"seeds: ");
        seeds = line["seeds: ".len()..]
            .split(" ")
            .map(|digits| digits.parse().unwrap())
            .collect::<Vec<_>>();

        let seeds_with_len = seeds.chunks(2);

        // seed_mapping = seeds.iter().map(|seed| SeedMapping::new(*seed)).collect();
        seed_mapping = seeds_with_len
            .map(|seed_and_len| SeedMapping::new(seed_and_len[0], seed_and_len[1]))
            .collect();
    }

    println!("seeds: {seeds:?}");
    println!("seeds_mapping first: {seed_mapping:?}");
    assert_eq!(lines_iter.next().unwrap(), *"");

    assert_eq!(lines_iter.next().unwrap(), *"seed-to-soil map:");
    let mut seed_mapping2 = vec![];
    let mut seed_mapping3: RangeMap<usize, usize> = RangeMap::new();
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.seed) {
                m.soil = dest;
                m.fertilizer = dest;
                m.water = dest;
                m.light = dest;
                m.temperature = dest;
                m.humidity = dest;
                m.location = dest;
            }
        }

        for s in &mut seed_mapping {
            let seed_start = s.seed;
            let seed_end = seed_start + s.len;
            let mapping_seed_start = awd.source;
            let mapping_seed_end = mapping_seed_start + awd.length;

            if (mapping_seed_start < seed_start && mapping_seed_end < seed_end)
                || (mapping_seed_start > seed_start && mapping_seed_end > seed_end)
            {
                println!("skip s: {s:?} awd: {awd:?}");
                continue;
            }
            /*
            if mapping_seed_end < seed_start || mapping_seed_start > seed_end
            {
                continue;
            }
            */

            let new_seed_start = seed_start.max(mapping_seed_start);
            let new_seed_end = seed_end.min(mapping_seed_end);
            let diff = seed_start as isize - mapping_seed_start as isize;
            // let diff = -diff;
            println!("seed_start: {seed_start}, new_seed_start: {new_seed_start}: mapping_seed_start: {mapping_seed_start}, diff: {diff}");
            let new_seed_len = new_seed_end - new_seed_start;
            let mut new = SeedMapping::new(new_seed_start, new_seed_len);
            let new_dest: usize = (awd.destination as isize + diff).try_into().unwrap();
            new.soil = new_dest;
            new.fertilizer = new_dest;
            new.water = new_dest;
            new.light = new_dest;
            new.temperature = new_dest;
            new.humidity = new_dest;
            new.location = new_dest;
            seed_mapping2.push(new);
        }
    }

    seed_mapping = seed_mapping2;
    println!("seeds_mapping second: {seed_mapping:?}");

    assert_eq!(lines_iter.next().unwrap(), *"soil-to-fertilizer map:");
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.soil) {
                m.fertilizer = dest;
                m.water = dest;
                m.light = dest;
                m.temperature = dest;
                m.humidity = dest;
                m.location = dest;
            }
        }
    }

    assert_eq!(lines_iter.next().unwrap(), *"fertilizer-to-water map:");
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.fertilizer) {
                m.water = dest;
                m.light = dest;
                m.temperature = dest;
                m.humidity = dest;
                m.location = dest;
            }
        }
    }

    assert_eq!(lines_iter.next().unwrap(), *"water-to-light map:");
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.water) {
                m.light = dest;
                m.temperature = dest;
                m.humidity = dest;
                m.location = dest;
            }
        }
    }

    assert_eq!(lines_iter.next().unwrap(), *"light-to-temperature map:");
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.light) {
                m.temperature = dest;
                m.humidity = dest;
                m.location = dest;
            }
        }
    }

    assert_eq!(lines_iter.next().unwrap(), *"temperature-to-humidity map:");
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.temperature) {
                println!("setting humid to {dest}");
                m.humidity = dest;
                m.location = dest;
            }
        }
    }

    assert_eq!(lines_iter.next().unwrap(), *"humidity-to-location map:");
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line
            .split(' ')
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect();

        let awd = MappingEntry {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        };

        for m in &mut seed_mapping {
            if let Some(dest) = awd.get_destination(&m.humidity) {
                println!("setting loc to {dest}");
                m.location = dest;
            }
        }
    }

    let lowest_location = seed_mapping.iter().map(|m| m.location).min().unwrap();

    println!("seeds: {:?}", seed_mapping.len());
    println!("seed_mapping: {seed_mapping:?}");
    println!("lowest_location: {lowest_location}");
}
