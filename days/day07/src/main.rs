use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone)]
enum FSNode {
    Directory(Rc<RefCell<FSDirectory>>),
    File(Rc<RefCell<FSFile>>),
}

struct FSDirectory {
    parent: Option<Rc<RefCell<FSDirectory>>>,
    name: String,
    children: Vec<FSNode>,
}

impl FSDirectory {
    fn new(parent: Option<Rc<RefCell<FSDirectory>>>, name: String) -> Self {
        FSDirectory {
            parent,
            name,
            children: Vec::new(),
        }
    }
}

struct FSFile {
    name: String,
    size: usize,
}

impl FSFile {
    fn new(name: String, size: usize) -> Self {
        FSFile { name, size }
    }
}

struct FileSystem {
    total_space: usize,
    root_dir: Rc<RefCell<FSDirectory>>,
    current_dir: Rc<RefCell<FSDirectory>>,
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = &self.root_dir.borrow();
        writeln!(f, "- {} (dir)", start.name)?;

        let children = start.children.clone();
        let mut depth_first_queue = VecDeque::from_iter(
            children
                .into_iter()
                .zip(vec![1_usize; start.children.len()]),
        );
        while !depth_first_queue.is_empty() {
            let (node, space_level) = depth_first_queue.pop_front().unwrap();
            let spaces = "  ".repeat(space_level);
            match node {
                FSNode::Directory(dir_rc) => {
                    writeln!(f, "{}- {} (dir)", spaces, dir_rc.borrow().name)?;

                    let children = dir_rc.borrow().children.clone();
                    for child in children.into_iter().rev() {
                        depth_first_queue.push_front((child, space_level + 1));
                    }
                }
                FSNode::File(file) => {
                    writeln!(
                        f,
                        "{}- {} (file, size={})",
                        spaces,
                        file.borrow().name,
                        file.borrow().size
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl FileSystem {
    fn new() -> Self {
        let root_dir = Rc::new(RefCell::new(FSDirectory::new(None, String::from("/"))));
        FileSystem {
            total_space: 70000000,
            root_dir: root_dir.clone(),
            current_dir: root_dir,
        }
    }

    fn find_subdirectory(&self, dir_name: &str) -> Option<Rc<RefCell<FSDirectory>>> {
        for child_node in &self.current_dir.borrow().children {
            match child_node {
                FSNode::Directory(child_dir) => {
                    if child_dir.borrow().name == dir_name {
                        return Some(child_dir.clone());
                    }
                }
                FSNode::File(_) => continue,
            }
        }

        None
    }

    fn create_directory(&mut self, dir_name: &str) -> Rc<RefCell<FSDirectory>> {
        let new_dir = Rc::new(RefCell::new(FSDirectory::new(
            Some(self.current_dir.clone()),
            dir_name.to_string(),
        )));
        self.current_dir
            .borrow_mut()
            .children
            .push(FSNode::Directory(new_dir.clone()));
        new_dir
    }

    fn create_file(&mut self, file_name: &str, file_size: usize) -> Rc<RefCell<FSFile>> {
        let new_file = Rc::new(RefCell::new(FSFile::new(file_name.to_string(), file_size)));
        self.current_dir
            .borrow_mut()
            .children
            .push(FSNode::File(new_file.clone()));

        new_file
    }

    fn change_directory(&mut self, dir_name: &str) {
        let next_dir = match dir_name {
            "/" => self.root_dir.clone(),
            ".." => self
                .current_dir
                .borrow()
                .parent
                .as_ref()
                .expect("Directory has no parent")
                .clone(),
            dir_name => {
                if let Some(dir) = self.find_subdirectory(dir_name) {
                    dir
                } else {
                    self.create_directory(dir_name)
                }
            }
        };

        self.current_dir = next_dir;
    }

    fn build_tree(&mut self, sh_lines: &[String]) {
        for sh_line in sh_lines {
            let mut sh_fields = sh_line.trim().split_ascii_whitespace();

            let field1 = sh_fields.next().expect("Missing first field");
            let field2 = sh_fields.next().expect("Missing second field");
            let field3_opt = sh_fields.next();
            match field1 {
                "$" => match field2 {
                    "cd" => {
                        let field3 = field3_opt.expect("Missing third field");
                        self.change_directory(field3);
                    }
                    "ls" => continue,
                    other => panic!("Unknown command {}", other),
                },
                "dir" => _ = self.create_directory(field2),
                number_str => {
                    let file_size = number_str.parse().expect("Failed to parse file size");
                    _ = self.create_file(field2, file_size);
                }
            }
        }
    }

    fn get_directory_size(&self, dir: &Rc<RefCell<FSDirectory>>) -> usize {
        let children = dir.borrow().children.clone();

        let mut depth_first_queue = VecDeque::from_iter(children.into_iter());
        let mut total_size = 0;
        while !depth_first_queue.is_empty() {
            let node = depth_first_queue.pop_front().unwrap();
            match node {
                FSNode::Directory(dir_rc) => {
                    let children = dir_rc.borrow().children.clone();
                    for child in children.into_iter().rev() {
                        depth_first_queue.push_front(child);
                    }
                }
                FSNode::File(file) => {
                    total_size += file.borrow().size;
                }
            }
        }

        total_size
    }

    fn get_all_directories(&self) -> Vec<Rc<RefCell<FSDirectory>>> {
        let children = self.root_dir.borrow().children.clone();

        let mut depth_first_queue = VecDeque::from_iter(children.into_iter());
        let mut directories = vec![self.root_dir.clone()];
        while !depth_first_queue.is_empty() {
            let node = depth_first_queue.pop_front().unwrap();
            match node {
                FSNode::Directory(dir_rc) => {
                    directories.push(dir_rc.clone());

                    let children = dir_rc.borrow().children.clone();
                    for child in children.into_iter().rev() {
                        depth_first_queue.push_front(child);
                    }
                }
                FSNode::File(_) => {
                    continue;
                }
            }
        }

        directories
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
    let sh_lines = parse_input("inputs/day07.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let mut file_system = FileSystem::new();
    file_system.build_tree(&sh_lines);
    let p1_file_sizes_sum: usize = file_system
        .get_all_directories()
        .into_iter()
        .map(|rc| file_system.get_directory_size(&rc))
        .filter(|&s| s < 100000)
        .sum();
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();

    let update_size = 30000000;
    let root_size = file_system.get_directory_size(&file_system.root_dir);
    let free_space_size = file_system.total_space - root_size;
    let required_free_size = update_size - free_space_size;
    let p2_freed_dir_size: usize = file_system
        .get_all_directories()
        .into_iter()
        .map(|rc| file_system.get_directory_size(&rc))
        .filter(|&s| s >= required_free_size)
        .min_by(|&s1, &s2| (s1 - required_free_size).cmp(&(s2 - required_free_size)))
        .unwrap();

    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 - File sizes sum: {}\n",
        part1_time, p1_file_sizes_sum
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 - Size of removed directory: {}\n",
        part2_time, p2_freed_dir_size
    );

    Ok(())
}
