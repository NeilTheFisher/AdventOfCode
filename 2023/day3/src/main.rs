use std::{
    fs::File,
    io::{prelude::*, BufReader},
    ops::{Add, AddAssign, Index, IndexMut, Neg},
};

#[derive(Debug)]
struct Node {
    value: u8,
    visited: bool,
}

type Grid = Vec<Vec<Node>>;

const ORIGIN: Point = Point::new(0, 0);
const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);
const UP_RIGHT: Point = Point::new(1, -1);
const UP_LEFT: Point = Point::new(-1, -1);
const DOWN_RIGHT: Point = Point::new(1, 1);
const DOWN_LEFT: Point = Point::new(-1, 1);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

trait Get {
    fn get(&self, p: Point) -> Option<&Node>;
    fn get_mut(&mut self, p: Point) -> Option<&mut Node>;
}

impl Get for Grid {
    fn get(&self, p: Point) -> Option<&Node> {
        if p.y >= self.len() as i32 || p.x >= self[0].len() as i32 || p.x < 0 || p.y < 0 {
            return None;
        }
        Some(&self[p])
    }

    fn get_mut(&mut self, p: Point) -> Option<&mut Node> {
        if p.y >= self.len() as i32 || p.x >= self[0].len() as i32 || p.x < 0 || p.y < 0 {
            return None;
        }
        Some(&mut self[p])
    }
}

impl Index<Point> for Grid {
    type Output = Node;

    fn index(&self, p: Point) -> &Self::Output {
        let x = p.x as usize;
        let y = p.y as usize;
        &self[y][x]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        let x = p.x as usize;
        let y = p.y as usize;
        &mut self[y][x]
    }
}

fn main() -> std::io::Result<()> {
    // let file = File::open("test_input.txt")?;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        grid.push(
            line.as_bytes()
                .iter()
                .map(|&b| Node {
                    value: b,
                    visited: false,
                })
                .collect(),
        );
    }

    let sum = sum_adjacent_numbers(&mut grid);

    dbg!(sum);

    Ok(())
}

fn sum_adjacent_numbers(grid: &mut Grid) -> i32 {
    let mut sum = 0;

    let mut y = 0;
    while y < grid.len() {
        let mut x = 0;
        while x < grid[0].len() {
            let p = Point::new(x as i32, y as i32);
            let node = &grid[p];

            if matches!(node.value, b'0'..=b'9' | b'.') {
                x += 1;
                continue;
            }

            // the value is a special char

            // check the adjacent points
            for mut new_p in [
                p + UP_LEFT,
                p + UP,
                p + UP_RIGHT,
                p + RIGHT,
                p + DOWN_RIGHT,
                p + DOWN,
                p + DOWN_LEFT,
                p + LEFT,
            ] {
                let node = grid.get_mut(new_p);
                if node.is_none() {
                    continue;
                }
                let mut node = node.unwrap();
                if !is_valid_node_value(&node) {
                    continue;
                }
                // find the start of the digits
                loop {
                    let new_node = grid.get(new_p + LEFT);
                    if let Some(new_node) = new_node {
                        new_p += LEFT;
                        if !is_valid_node_value(&new_node) {
                            break;
                        }
                    } else {
                        new_p += LEFT;
                        break;
                    }
                }
                // put those digits in a string
                let mut s = String::new();
                loop {
                    let new_node = grid.get_mut(new_p + RIGHT);
                    if let Some(new_node) = new_node {
                        new_p += RIGHT;
                        if is_valid_node_value(&new_node) {
                            node = new_node;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    s.push(node.value as char);
                    node.visited = true;
                }
                sum += s.parse::<i32>().unwrap();
            }

            x += 1;
        }

        y += 1;
    }

    sum
}

fn is_valid_node_value(node: &Node) -> bool {
    !node.visited && matches!(node.value, b'0'..=b'9')
}
