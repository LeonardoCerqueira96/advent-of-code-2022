use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

mod dijkstra {
    #[derive(Debug)]
    pub struct DijkstraNode {
        pub position: (usize, usize),
        pub parent: Option<(usize, usize)>,
        pub f: f64,
    }

    impl PartialEq for DijkstraNode {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position
                && self.parent == other.parent
                && (self.f - other.f).abs() < 1e-10
        }
    }

    impl Eq for DijkstraNode {}

    impl Ord for DijkstraNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.f.total_cmp(&self.f)
        }
    }

    impl PartialOrd for DijkstraNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Vec<u32>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn new(heights: Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Self {
        HeightMap {
            heights,
            start,
            end,
        }
    }

    fn get_higher_neighbours(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut higher_neighbours = Vec::new();
        let pos_height = self.heights[position.0][position.1];

        // North
        if position.0 > 0 && self.heights[position.0 - 1][position.1] <= pos_height + 1 {
            higher_neighbours.push((position.0 - 1, position.1));
        }

        // South
        if position.0 < self.heights.len() - 1
            && self.heights[position.0 + 1][position.1] <= pos_height + 1
        {
            higher_neighbours.push((position.0 + 1, position.1));
        }

        // West
        if position.1 > 0 && self.heights[position.0][position.1 - 1] <= pos_height + 1 {
            higher_neighbours.push((position.0, position.1 - 1));
        }

        // East
        if position.1 < self.heights[0].len() - 1
            && self.heights[position.0][position.1 + 1] <= pos_height + 1
        {
            higher_neighbours.push((position.0, position.1 + 1));
        }

        higher_neighbours
    }

    fn get_lower_neighbours(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut lower_neighbours = Vec::new();
        let pos_height = self.heights[position.0][position.1];

        // North
        if position.0 > 0
            && self.heights[position.0 - 1][position.1] >= pos_height.saturating_sub(1)
        {
            lower_neighbours.push((position.0 - 1, position.1));
        }

        // South
        if position.0 < self.heights.len() - 1
            && self.heights[position.0 + 1][position.1] >= pos_height.saturating_sub(1)
        {
            lower_neighbours.push((position.0 + 1, position.1));
        }

        // West
        if position.1 > 0
            && self.heights[position.0][position.1 - 1] >= pos_height.saturating_sub(1)
        {
            lower_neighbours.push((position.0, position.1 - 1));
        }

        // East
        if position.1 < self.heights[0].len() - 1
            && self.heights[position.0][position.1 + 1] >= pos_height.saturating_sub(1)
        {
            lower_neighbours.push((position.0, position.1 + 1));
        }

        lower_neighbours
    }

    fn calculate_start_end_path(&self) -> Vec<((usize, usize), u32)> {
        // Priority queue of nodes
        let mut open_list = BinaryHeap::new();

        // Map of nodes that have already been checked
        let mut closed_map = HashMap::new();

        // Initialize start node
        let start_node = dijkstra::DijkstraNode {
            position: self.start,
            parent: None,
            f: 0.,
        };

        open_list.push(start_node);
        while let Some(node) = open_list.pop() {
            let node_pos = node.position;
            for neighbour_pos in self.get_higher_neighbours(node_pos) {
                // Skip this node if it has already been closed
                if closed_map.contains_key(&neighbour_pos) {
                    continue;
                }

                // If the same node is already in the open list, but with a lower cost, skip this node
                let neighbour_f = node.f + 1.;
                let min_f = open_list
                    .iter()
                    .filter(|&node| node.position == neighbour_pos)
                    .map(|node| node.f)
                    .min_by(|f1, f2| f1.total_cmp(f2));
                if let Some(f) = min_f {
                    if f <= neighbour_f {
                        continue;
                    }
                }

                // Build new node
                let neighbour_node = dijkstra::DijkstraNode {
                    position: neighbour_pos,
                    parent: Some(node_pos),
                    f: neighbour_f,
                };

                open_list.push(neighbour_node);
            }

            // We're done with this node
            closed_map.insert(node_pos, node);

            // Stop if we've reached the end
            if node_pos == self.end {
                break;
            }
        }

        // Rebuild the path by reverse iterating through the nodes' parents
        let mut path = Vec::new();
        let mut path_node_pos = self.end;
        while let Some(path_node) = closed_map.get(&path_node_pos) {
            let pos_height = self.heights[path_node_pos.0][path_node_pos.1];
            path.insert(0, (path_node_pos, pos_height));

            if let Some(previous_pos) = path_node.parent {
                path_node_pos = previous_pos;
            } else {
                break;
            }
        }

        path
    }

    fn calculate_shortest_hike_path(&self) -> Vec<((usize, usize), u32)> {
        // Priority queue of nodes
        let mut open_list = BinaryHeap::new();

        // Map of nodes that have already been checked
        let mut closed_map = HashMap::new();

        // Initialize start node
        // We start and the "end" node because we want to find the path to nearest height 'a'
        let start_node = dijkstra::DijkstraNode {
            position: self.end,
            parent: None,
            f: 0.,
        };

        // Variable to hold the position of the start of the hike path
        let mut target_pos = (0, 0);

        open_list.push(start_node);
        while let Some(node) = open_list.pop() {
            let node_pos = node.position;
            for neighbour_pos in self.get_lower_neighbours(node_pos) {
                // Skip this node if it has already been closed
                if closed_map.contains_key(&neighbour_pos) {
                    continue;
                }

                // If the same node is already in the open list, but with a lower cost, skip this node
                let neighbour_f = node.f + 1.;
                let min_f = open_list
                    .iter()
                    .filter(|&node| node.position == neighbour_pos)
                    .map(|node| node.f)
                    .min_by(|f1, f2| f1.total_cmp(f2));
                if let Some(f) = min_f {
                    if f <= neighbour_f {
                        continue;
                    }
                }

                // Build new node
                let neighbour_node = dijkstra::DijkstraNode {
                    position: neighbour_pos,
                    parent: Some(node_pos),
                    f: neighbour_f,
                };

                open_list.push(neighbour_node);
            }

            closed_map.insert(node_pos, node);

            // If we reached a node of height 0, stop and save the position in the variable
            let node_height = self.heights[node_pos.0][node_pos.1];
            if node_height == 0 {
                target_pos = node_pos;
                break;
            }
        }

        // Rebuild the path by reverse iterating through the nodes' parents
        let mut path = Vec::new();
        let mut path_node_pos = target_pos;
        while let Some(path_node) = closed_map.get(&path_node_pos) {
            let pos_height = self.heights[path_node_pos.0][path_node_pos.1];
            path.push((path_node_pos, pos_height));

            if let Some(previous_pos) = path_node.parent {
                path_node_pos = previous_pos;
            } else {
                break;
            }
        }

        path
    }
}

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<HeightMap> {
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let mut height_rows = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input_buf.lines().enumerate() {
        let line = line?;

        let row = line
            .chars()
            .enumerate()
            .map(|(j, c)| match c {
                'S' => {
                    start = (i, j);
                    0
                }
                'E' => {
                    end = (i, j);
                    25
                }
                h => h as u32 - 97,
            })
            .collect();

        height_rows.push(row);
    }

    Ok(HeightMap::new(height_rows, start, end))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let height_map = parse_input("inputs/day12.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let p1_shortest_path = height_map.calculate_start_end_path();
    let p1_steps_count = p1_shortest_path.len() - 1;
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let p2_shortest_hike_path = height_map.calculate_shortest_hike_path();
    let p2_steps_count = p2_shortest_hike_path.len() - 1;
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nMininum steps to reach the end: {}\n",
        part1_time, p1_steps_count
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nShortest hike path length: {}\n",
        part2_time, p2_steps_count
    );

    Ok(())
}
