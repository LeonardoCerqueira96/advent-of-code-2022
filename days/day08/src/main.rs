use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

struct TreeGrid {
    width: usize,
    height: usize,
    grid: Vec<Vec<usize>>,
}

impl TreeGrid {
    fn new(grid: Vec<Vec<usize>>) -> Self {
        assert!(!grid.is_empty());

        let height = grid.len();
        let width = grid[0].len();
        assert!(grid.iter().all(|r| r.len() == width));

        TreeGrid {
            width,
            height,
            grid,
        }
    }

    fn get_visible_trees_count(&self) -> usize {
        let mut visible_count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                // Test if the tree is on the edge of the grid
                let is_on_edge = i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1;
                if is_on_edge {
                    visible_count += 1;
                    continue;
                }

                let cur_height = self.grid[i][j];

                // Check trees to the north
                let mut north_visible = true;
                for pos_i in 0..i {
                    if self.grid[pos_i][j] >= cur_height {
                        north_visible = false;
                        break;
                    }
                }
                if north_visible {
                    visible_count += 1;
                    continue;
                }

                // Check trees to the east
                let mut east_visible = true;
                for pos_j in j + 1..self.width {
                    if self.grid[i][pos_j] >= cur_height {
                        east_visible = false;
                        break;
                    }
                }
                if east_visible {
                    visible_count += 1;
                    continue;
                }

                // Check trees to the south
                let mut south_visible = true;
                for pos_i in i + 1..self.height {
                    if self.grid[pos_i][j] >= cur_height {
                        south_visible = false;
                        break;
                    }
                }
                if south_visible {
                    visible_count += 1;
                    continue;
                }

                // Check trees to the west
                let mut west_visible = true;
                for pos_j in 0..j {
                    if self.grid[i][pos_j] >= cur_height {
                        west_visible = false;
                        break;
                    }
                }
                if west_visible {
                    visible_count += 1;
                    continue;
                }
            }
        }

        visible_count
    }

    fn get_view_scores(&self) -> Vec<Vec<usize>> {
        let mut view_scores = vec![vec![0; self.width]; self.height];
        for (i, score_row) in view_scores.iter_mut().enumerate().take(self.height) {
            for (j, score) in score_row.iter_mut().enumerate().take(self.width) {
                // Test if the tree is on the edge of the grid
                let is_on_edge = i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1;
                if is_on_edge {
                    *score = 0;
                    continue;
                }

                let cur_height = self.grid[i][j];

                // Calculate north viewing score
                let mut north_score = 0;
                for pos_i in (0..i).rev() {
                    north_score += 1;
                    if self.grid[pos_i][j] >= cur_height {
                        break;
                    }
                }

                // Calculate east viewing score
                let mut east_score = 0;
                for pos_j in j + 1..self.width {
                    east_score += 1;
                    if self.grid[i][pos_j] >= cur_height {
                        break;
                    }
                }

                // Calculate south viewing score
                let mut south_score = 0;
                for pos_i in i + 1..self.height {
                    south_score += 1;
                    if self.grid[pos_i][j] >= cur_height {
                        break;
                    }
                }

                // Calculate west viewing score
                let mut west_score = 0;
                for pos_j in (0..j).rev() {
                    west_score += 1;
                    if self.grid[i][pos_j] >= cur_height {
                        break;
                    }
                }

                // Calculate total viewing score
                *score = north_score * east_score * south_score * west_score;
            }
        }

        view_scores
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<Vec<Vec<usize>>> {
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let mut tree_grid = Vec::new();
    for line in input_buf.lines() {
        let line = line?;

        let heights_row: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10).expect("Failed to parse digit") as usize)
            .collect();
        tree_grid.push(heights_row);
    }

    Ok(tree_grid)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let tree_grid = TreeGrid::new(parse_input("inputs/day08.in")?);
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let p1_visible_count = tree_grid.get_visible_trees_count();
    let part1_time = t1.elapsed();

    // // Compute part 2 and time it
    let t2 = Instant::now();
    let p2_max_view_score = *tree_grid.get_view_scores().iter().flatten().max().unwrap();
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 - Amount of visible trees: {}\n",
        part1_time, p1_visible_count
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 - Max view score: {}\n",
        part2_time, p2_max_view_score
    );

    Ok(())
}
