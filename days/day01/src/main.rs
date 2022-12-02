use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn parse_input<T: AsRef<Path>>(filename: T, do_sort: bool) -> io::Result<Vec<Vec<u64>>> {
    let mut elves_calories = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line in input_buf.lines() {
        let line = line?;

        // Push first elf
        if elves_calories.is_empty() {
            elves_calories.push(Vec::new());
        }

        // If it's an empty line, we start a new elf
        if line.is_empty() {
            elves_calories.push(Vec::new());
            continue;
        }

        let calories_count = line
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        elves_calories.last_mut().unwrap().push(calories_count);
    }

    if do_sort {
        elves_calories.sort_by(|x, y| {
            let x_sum: u64 = x.iter().sum();
            let y_sum: u64 = y.iter().sum();
            y_sum.cmp(&x_sum)
        });
    }

    Ok(elves_calories)
}

fn part1(elves_calories: &[Vec<u64>]) -> u64 {
    elves_calories.first().unwrap().iter().sum()
}

fn part2(elves_calories: &[Vec<u64>]) -> (u64, u64, u64) {
    let max1 = elves_calories[0].iter().sum();
    let max2 = elves_calories[1].iter().sum();
    let max3 = elves_calories[2].iter().sum();

    (max1, max2, max3)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input, sort it and time it
    let t0 = Instant::now();
    let elves_calories = parse_input("inputs/day01.in", true)?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let max_calories = part1(&elves_calories);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let (max1, max2, max3) = part2(&elves_calories);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_millis() as f64 + parse_time.subsec_nanos() as f64 * 1e-6;
    println!("Parsing and sorting the input took {:.6}ms\n", parse_time);

    let part1_time = part1_time.as_millis() as f64 + part1_time.subsec_nanos() as f64 * 1e-6;
    println!(
        "Part 1:\nTook {:.6}ms\nMax calories: {}\n",
        part1_time, max_calories
    );

    let part2_time = part2_time.as_millis() as f64 + part2_time.subsec_nanos() as f64 * 1e-6;
    println!(
        "Part 2:\nTook {:.6}ms\nSum of three largest max calories: {}\n",
        part2_time,
        max1 + max2 + max3
    );

    Ok(())
}
