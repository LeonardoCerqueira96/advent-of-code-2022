use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

type RangePair = ((u64, u64), (u64, u64));
fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<Vec<RangePair>> {
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let range_pairs: io::Result<Vec<_>> = input_buf
        .lines()
        .map(|rp| {
            let rp = rp?;

            // Split line to get range pairs
            let mut pair_str = rp.split(',').take(2);
            let (range1_str, range2_str) = (pair_str.next().unwrap(), pair_str.next().unwrap());

            // Range 1
            let mut range1_it = range1_str.split('-').take(2);
            let (start1, end1): (u64, u64) = (
                range1_it.next().unwrap().parse().unwrap(),
                range1_it.next().unwrap().parse().unwrap(),
            );

            // Range 2
            let mut range2_it = range2_str.split('-').take(2);
            let (start2, end2): (u64, u64) = (
                range2_it.next().unwrap().parse().unwrap(),
                range2_it.next().unwrap().parse().unwrap(),
            );

            Ok(((start1, end1), (start2, end2)))
        })
        .collect();

    range_pairs
}

fn ranges_fully_overlap(range_pair: &RangePair) -> bool {
    // Check if the first range fully contains the second
    if range_pair.1 .0 >= range_pair.0 .0 && range_pair.1 .1 <= range_pair.0 .1 {
        return true;
    }

    // Check if the second range fully contains the first
    if range_pair.0 .0 >= range_pair.1 .0 && range_pair.0 .1 <= range_pair.1 .1 {
        return true;
    }

    false
}

fn ranges_partially_overlap(range_pair: &RangePair) -> bool {
    // Check if the first range overlaps with the second
    if (range_pair.1 .0 >= range_pair.0 .0 && range_pair.1 .0 <= range_pair.0 .1)
        || (range_pair.1 .1 >= range_pair.0 .0 && range_pair.1 .1 <= range_pair.0 .1)
    {
        return true;
    }

    // Check if the second range overlaps with the first
    if (range_pair.0 .0 >= range_pair.1 .0 && range_pair.0 .0 <= range_pair.1 .1)
        || (range_pair.0 .1 >= range_pair.1 .0 && range_pair.0 .1 <= range_pair.1 .1)
    {
        return true;
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let range_pairs = parse_input("inputs/day04.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let p1_overlap_count = range_pairs
        .iter()
        .filter(|&p| ranges_fully_overlap(p))
        .count();
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let p2_overlap_count = range_pairs
        .iter()
        .filter(|&p| ranges_partially_overlap(p))
        .count();
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 overlap count: {}\n",
        part1_time, p1_overlap_count
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 overlap count: {}\n",
        part2_time, p2_overlap_count
    );

    Ok(())
}
