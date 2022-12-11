use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::time::Instant;

use num::Integer;
use regex::Regex;

const LINE_ENDING_WIN: &str = "\r\n";
const LINE_ENDING_UNX: &str = "\n";

#[derive(Debug, Clone)]
enum InspectOperation {
    Add(usize),
    Mult(usize),
    Pow,
}

#[derive(Debug, Clone)]
struct ThrowCheck {
    modulo: usize,
    if_true_monkey: usize,
    if_false_monkey: usize,
}

impl ThrowCheck {
    fn new(modulo: usize, if_true_monkey: usize, if_false_monkey: usize) -> Self {
        ThrowCheck {
            modulo,
            if_true_monkey,
            if_false_monkey,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    inspect_op: InspectOperation,
    throw_check: ThrowCheck,
    inspect_count: usize,
}

impl Monkey {
    fn new(items: Vec<usize>, inspect_op: InspectOperation, throw_check: ThrowCheck) -> Self {
        Monkey {
            items,
            inspect_op,
            throw_check,
            inspect_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyPack {
    monkeys: Vec<Monkey>,
    global_lcm: usize,
}

impl MonkeyPack {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let global_lcm = monkeys.iter().map(|m| m.throw_check.modulo).fold(1, |acc, m| acc.lcm(&m));

        MonkeyPack {
            monkeys,
            global_lcm,
        }
    }

    fn run_one_round(&mut self, divide_worry_level: bool) {
        for monkey_index in 0..self.monkeys.len() {
            let modulo = self.monkeys[monkey_index].throw_check.modulo;

            while !self.monkeys[monkey_index].items.is_empty() {
                // Get next item
                let mut worry_lvl = self.monkeys[monkey_index].items.remove(0);

                // Do inspect operation to increase worry level
                worry_lvl = match self.monkeys[monkey_index].inspect_op {
                    InspectOperation::Add(n) => (worry_lvl + n) % self.global_lcm,
                    InspectOperation::Mult(n) => (worry_lvl * n) % self.global_lcm,
                    InspectOperation::Pow => (worry_lvl * worry_lvl) % self.global_lcm,
                };

                // Increment inspeect counter
                self.monkeys[monkey_index].inspect_count += 1;

                if divide_worry_level {
                    // Monkey gets bored, divide worry level by three
                    worry_lvl = (worry_lvl / 3) % self.global_lcm;
                }

                // Check which monkey to throw to
                let monkey_thrown_to = if worry_lvl % modulo == 0 {
                    self.monkeys[monkey_index].throw_check.if_true_monkey
                } else {
                    self.monkeys[monkey_index].throw_check.if_false_monkey
                };

                // Throw item
                self.monkeys[monkey_thrown_to].items.push(worry_lvl);
            }
        }
    }

    fn get_two_most_active_monkeys(&self) -> (&Monkey, &Monkey) {
        let mut monkey_refs: Vec<_> = self.monkeys.iter().collect();
        monkey_refs.sort_by(|&m_a, &m_b| m_b.inspect_count.cmp(&m_a.inspect_count));

        (monkey_refs[0], monkey_refs[1])
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<MonkeyPack> {
    // Setup regexes
    let items_re = Regex::new(r"Starting\s+items:\s*((?:\d+(?:,\s*)?)+)")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let operation_re = Regex::new(r"Operation:\s*new\s*=\s*old\s*([+*]\s*(?:(?:old)|(?:\d+)))")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let test_re = Regex::new(r"Test:\s*divisible\s+by\s+(\d+)")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let if_true_re = Regex::new(r"If\s+true:\s*throw\s+to\s+monkey\s+(\d+)")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let if_false_re = Regex::new(r"If\s+false:\s*throw\s+to\s+monkey\s+(\d+)")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Open input file
    let input = File::open(filename)?;
    let mut input_buf = BufReader::new(input);

    let mut input_str = String::new();
    input_buf.read_to_string(&mut input_str)?;

    // Choose line ending
    let line_ending = if input_str.contains(LINE_ENDING_WIN) {
        LINE_ENDING_WIN
    } else {
        LINE_ENDING_UNX
    };

    let mut monkeys = Vec::new();
    for monkey_str in input_str.split(&line_ending.repeat(2)) {
        let monkey_lines: Vec<_> = monkey_str.lines().skip(1).collect();

        // Parse items
        let items_caps = items_re
            .captures(monkey_lines[0])
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to match items"))?;

        let items_str = (items_caps.get(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to capture items string",
            )
        })?)
        .as_str();

        let items: Result<Vec<usize>, _> = items_str.split(',').map(|n| n.trim().parse()).collect();
        let items = items.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // Parse operation
        let operation_cap = operation_re.captures(monkey_lines[1]).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Failed to match operation")
        })?;

        let operation_str = (operation_cap.get(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to capture operation string",
            )
        })?)
        .as_str();

        let operation_fields: Vec<_> = operation_str.split_ascii_whitespace().take(2).collect();
        let operation = match (operation_fields[0], operation_fields[1]) {
            ("+", num_str) => {
                let num = num_str
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                InspectOperation::Add(num)
            }
            ("*", "old") => InspectOperation::Pow,
            ("*", num_str) => {
                let num = num_str
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                InspectOperation::Mult(num)
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unable to parse operation",
                ))
            }
        };

        // Parse test
        let test_cap = test_re
            .captures(monkey_lines[2])
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to match test"))?;

        let test_str = (test_cap.get(1).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Failed to capture test string")
        })?)
        .as_str();

        let test = test_str
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // Parse true case
        let if_true_cap = if_true_re.captures(monkey_lines[3]).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Failed to match if true case")
        })?;
        let if_true_str = (if_true_cap.get(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to capture if true string",
            )
        })?)
        .as_str();

        let if_true = if_true_str
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // Parse false case
        let if_false_cap = if_false_re.captures(monkey_lines[4]).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Failed to match if false case")
        })?;
        let if_false_str = (if_false_cap.get(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to capture if false string",
            )
        })?)
        .as_str();

        let if_false = if_false_str
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // Build monkey
        let monkey = Monkey::new(items, operation, ThrowCheck::new(test, if_true, if_false));
        monkeys.push(monkey);
    }

    Ok(MonkeyPack::new(monkeys))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let mut monkey_pack_p1 = parse_input("inputs/day11.in")?;
    let mut monkey_pack_p2 = monkey_pack_p1.clone();
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    for _ in 0..20 {
        monkey_pack_p1.run_one_round(true);
    }
    let p1_monkey_business = {
        let p1_two_most_active_monkeys = monkey_pack_p1.get_two_most_active_monkeys();
        p1_two_most_active_monkeys.0.inspect_count * p1_two_most_active_monkeys.1.inspect_count
    };
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    for _ in 0..10000 {
        monkey_pack_p2.run_one_round(false);
    }
    let p2_monkey_business = {
        let p2_two_most_active_monkeys = monkey_pack_p2.get_two_most_active_monkeys();
        p2_two_most_active_monkeys.0.inspect_count * p2_two_most_active_monkeys.1.inspect_count
    };
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 - Monkey business: {}\n",
        part1_time, p1_monkey_business
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 - Monkey business: {}\n",
        part2_time, p2_monkey_business
    );

    Ok(())
}
