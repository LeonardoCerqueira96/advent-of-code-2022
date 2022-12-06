use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant;

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<Vec<char>> {
    // Open input file
    let input = File::open(filename)?;
    let mut input_buf = BufReader::new(input);

    let mut stream = String::new();
    input_buf.read_to_string(&mut stream)?;

    Ok(stream.chars().collect())
}

fn find_first_marker(stream: &[char]) -> usize {
    // Iterate over all 4 character windows
    for (i, window) in stream.windows(4).enumerate() {
        // Check if all 4 characters are unique using a hash set
        let mut charset = HashSet::new();
        if window.iter().all(|c| charset.insert(c)) {
            return i + 4;
        }
    }

    usize::MAX
}

fn find_first_message(stream: &[char]) -> usize {
    // Iterate over all 14 character windows
    for (i, window) in stream.windows(14).enumerate() {
        // Check if all 14 characters are unique using a hash set
        let mut charset = HashSet::new();
        if window.iter().all(|c| charset.insert(c)) {
            return i + 14;
        }
    }

    usize::MAX
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let stream = parse_input("inputs/day06.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let p1_first_marker_pos = find_first_marker(&stream);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let p2_first_message_pos = find_first_message(&stream);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 - First marker position: {}\n",
        part1_time, p1_first_marker_pos
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 - First message position: {}\n",
        part2_time, p2_first_message_pos
    );

    Ok(())
}
