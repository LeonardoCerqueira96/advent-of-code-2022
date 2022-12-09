use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use std::{io, vec};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "u" => Ok(Direction::Up),
            "D" | "d" => Ok(Direction::Down),
            "L" | "l" => Ok(Direction::Left),
            "R" | "r" => Ok(Direction::Right),
            other => Err(format!("Can't convert string '{}' to direction", other)),
        }
    }
}

struct RopeGrid {
    segments: Vec<(isize, isize)>,
    tail_visited_set: HashSet<(isize, isize)>,
}

impl RopeGrid {
    fn new(n_segments: usize) -> Self {
        let segments = vec![(0, 0); n_segments];

        let mut tail_visited_set = HashSet::new();
        tail_visited_set.insert((0, 0));

        RopeGrid {
            segments,
            tail_visited_set,
        }
    }

    fn do_movement(&mut self, movement: &(Direction, usize)) {
        let measure_distance = |pos1: (isize, isize), pos2: (isize, isize)| {
            (pos1.0 - pos2.0).pow(2) + (pos1.1 - pos2.1).pow(2)
        };

        let move_offset = match movement.0 {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..movement.1 {
            self.segments[0].0 += move_offset.0;
            self.segments[0].1 += move_offset.1;

            for i in 1..self.segments.len() {
                let segment_is_tail = i == self.segments.len() - 1;

                let distance = measure_distance(self.segments[i - 1], self.segments[i]);
                if distance >= 4 {
                    let diff = (
                        (self.segments[i - 1].0 - self.segments[i].0).signum(),
                        (self.segments[i - 1].1 - self.segments[i].1).signum(),
                    );
                    self.segments[i].0 += diff.0;
                    self.segments[i].1 += diff.1;
                }

                if segment_is_tail {
                    self.tail_visited_set.insert(self.segments[i]);
                }
            }
        }
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<Vec<(Direction, usize)>> {
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let mut movements = Vec::new();
    for line in input_buf.lines() {
        let line = line?;
        let mut fields_it = line.split_ascii_whitespace().take(2);

        // Parse direction
        let direction_str = fields_it.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expect direction field not found",
            )
        })?;
        let direction = Direction::from_str(direction_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // Parse steps
        let steps_str = fields_it.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Expect steps field not found")
        })?;
        let steps = steps_str
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        movements.push((direction, steps));
    }

    Ok(movements)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let movements = parse_input("inputs/day09.in")?;
    let mut rope_grid_p1 = RopeGrid::new(2);
    let mut rope_grid_p2 = RopeGrid::new(10);
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    for movement in &movements {
        rope_grid_p1.do_movement(movement);
    }
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    for movement in &movements {
        rope_grid_p2.do_movement(movement);
    }
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 - Amount of positions visited by the rope's tail: {}\n",
        part1_time,
        rope_grid_p1.tail_visited_set.len()
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 - Amount of positions visited by the rope's tail: {}\n",
        part2_time,
        rope_grid_p2.tail_visited_set.len()
    );

    Ok(())
}
