use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn find_common_item(items: &str) -> char {
    // Iterate over the items of the first container, and return the one that is also in the second container
    let (first_container, second_container) = items.split_at(items.len() / 2);
    let mut common_item = '\0';
    for item in first_container.chars() {
        if second_container.find(item).is_some() {
            common_item = item;
            break;
        }
    }

    common_item
}

fn find_group_badges(rucksacks: &[String]) -> Vec<char> {
    // Iterate over groups of 3 rucksacks and map each group to their common item
    rucksacks
        .chunks(3)
        .map(|group| {
            for item in group[0].chars() {
                if group[1].find(item).is_some() && group[2].find(item).is_some() {
                    return item;
                }
            }
            // If we get to this panic, the input is faulty
            panic!("Could not find the common item in the group of three!");
        })
        .collect()
}

fn get_priority(item: char) -> u64 {
    // Use ASCII table to easily calculate the priority of an item
    if item.is_ascii_lowercase() {
        item as u64 - 96
    } else if item.is_ascii_uppercase() {
        item as u64 - 38
    } else {
        // If we get to this panic, the input is faulty
        panic!("Item {} is not an ASCII character!", item);
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<Vec<String>> {
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    input_buf.lines().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let rucksacks = parse_input("inputs/day03.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let p1_priorities_sum: u64 = rucksacks
        .iter()
        .map(|r| {
            let common_item = find_common_item(r);
            get_priority(common_item)
        })
        .sum();
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let badges = find_group_badges(&rucksacks);
    let p2_priorities_sum: u64 = badges.iter().map(|&b| get_priority(b)).sum();
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 priorities sum: {}\n",
        part1_time, p1_priorities_sum
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 priorities sum: {}\n",
        part2_time, p2_priorities_sum
    );

    Ok(())
}
