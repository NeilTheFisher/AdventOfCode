use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Index, IndexMut, Neg},
};

const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);
const ORIGIN: Point = Point::new(0, 0);

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

type Grid = Vec<Vec<u8>>;

trait Get {
    fn get(&self, p: Point) -> Option<u8>;
}

impl Get for Grid {
    fn get(&self, p: Point) -> Option<u8> {
        if p.y > self.len() as i32 || p.x > self[0].len() as i32 || p.x < 0 || p.y < 0 {
            return None;
        }
        Some(self[p])
    }
}

impl Index<Point> for Grid {
    type Output = u8;

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

        grid.push(line.as_bytes().to_vec());
    }

    get_enclosed_count(grid);

    Ok(())
}

fn get_enclosed_count(grid: Grid) {
    let mut pos = find_start_pos(&grid);
    let mut last_direction = ORIGIN;

    let mut visited_grid: Grid = vec![vec![0; grid[0].len()]; grid.len()];

    let mut steps = 1;

    while let Some(next_dir) = find_next_direction(&grid, pos, -last_direction) {
        pos += next_dir;
        visited_grid[pos] = 1;
        steps += 1;
        last_direction = next_dir;
    }

    steps /= 2;
    dbg!(steps);

    let mut enclosed_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited_grid[i][j] == 0 {
                // if a point is inside of a shape, it'll cross its edges an odd number of times.
                let crossings = count_edge_crossings(&grid, &visited_grid, i, j);
                if crossings % 2 == 1 {
                    enclosed_count += 1;
                }
            }
        }
    }

    dbg!(enclosed_count);
}

fn count_edge_crossings(grid: &Grid, visited: &Grid, i: usize, j: usize) -> i32 {
    let mut count = 0;

    for k in 0..j {
        // J, L, | define edges
        if visited[i][k] == 1 && matches!(grid[i][k], b'J' | b'L' | b'|') {
            count += 1;
        }
    }

    count
}

fn find_start_pos(grid: &Grid) -> Point {
    for (i, rows) in grid.iter().enumerate() {
        for (j, &c) in rows.iter().enumerate() {
            if c == b'S' {
                return Point::new(j as i32, i as i32);
            }
        }
    }
    unreachable!();
}

fn find_next_direction(grid: &Grid, start: Point, ignore: Point) -> Option<Point> {
    if ignore != UP
        && matches!(grid.get(start), Some(b'S' | b'|' | b'L' | b'J'))
        && matches!(grid.get(start + UP), Some(b'|' | b'7' | b'F'))
    {
        return Some(UP);
    }
    if ignore != DOWN
        && matches!(grid.get(start), Some(b'S' | b'|' | b'7' | b'F'))
        && matches!(grid.get(start + DOWN), Some(b'|' | b'L' | b'J'))
    {
        return Some(DOWN);
    }
    if ignore != LEFT
        && matches!(grid.get(start), Some(b'S' | b'-' | b'7' | b'J'))
        && matches!(grid.get(start + LEFT), Some(b'-' | b'L' | b'F'))
    {
        return Some(LEFT);
    }
    if ignore != RIGHT
        && matches!(grid.get(start), Some(b'S' | b'-' | b'L' | b'F'))
        && matches!(grid.get(start + RIGHT), Some(b'-' | b'7' | b'J'))
    {
        return Some(RIGHT);
    }

    None
}
