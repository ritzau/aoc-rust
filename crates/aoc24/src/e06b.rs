use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleResult};
use fxhash::FxHashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::max;

const DAY: Day = Day(6);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Guard Gallivant");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input.read_to_string()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 4665);

    let p2 = part2(&input.read_to_string()?)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 1688);

    Ok(())
}

fn part1(_input: &str) -> PuzzleResult<usize> {
    Ok(0)
}

fn part2(_input: &str) -> PuzzleResult<usize> {
    Ok(0)
}

fn parse(input: &str) -> (Grid, (usize, usize)) {
    let max_size = input.lines().count();
    let mut grid = Grid::new(max_size);

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(col, _)| {
                grid.add_obstacle(row, col);
            });
    });

    // println!("{:?}", grid);

    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| {
            line.chars()
                .enumerate()
                .find(|(_, c)| *c == '^')
                .map(|(col, _)| (row, col))
        })
        .unwrap();

    (grid, start_pos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    rows: Vec<Vec<usize>>, // Obstacle positions for each row
    cols: Vec<Vec<usize>>, // Obstacle positions for each column
    max_size: usize,       // Grid size
    next_obstacles: Vec<Vec<(usize, usize, usize, usize)>>, // Indices for (North, South, West, East)
}

impl Grid {
    fn new(max_size: usize) -> Self {
        Grid {
            rows: vec![Vec::new(); max_size],
            cols: vec![Vec::new(); max_size],
            max_size,
            next_obstacles: vec![
                vec![(usize::MAX, usize::MAX, usize::MAX, usize::MAX); max_size];
                max_size
            ],
        }
    }

    fn add_obstacle(&mut self, row: usize, col: usize) {
        self.rows[row].push(col);
        self.cols[col].push(row);
    }

    fn generate_next_obstacles(&mut self) {
        let obstacles: Vec<(usize, usize)> = vec![(1, 2), (3, 0), (0, 3), (2, 1)];
        let max_r = obstacles.iter().map(|(r, _)| r).max().unwrap();
        let max_c = obstacles.iter().map(|(_, c)| c).max().unwrap();
        let max_size = max(*max_r, *max_c);
        let mut rows = vec![Vec::<usize>::new(); max_size];
        let mut cols = vec![Vec::<usize>::new(); max_size];
        obstacles.iter().copied().for_each(|(r, c)| {
            rows[r].push(c);
            cols[c].push(r);
        });
        rows.iter_mut().for_each(|r| r.sort_unstable());
        cols.iter_mut().for_each(|c| c.sort_unstable());

        let mut grid = [[(0usize, 0usize); 10]; 10];
        for (row, col) in obstacles.iter().copied() {
            if row > 0 {
                // Hit from north, turning west
                let col_next = rows[row - 1]
                    .iter()
                    .copied()
                    .filter(|&other| other < col)
                    .last()
                    .unwrap_or(usize::MAX);
                grid[row][col] = (row, col_next);
            }
        }
    }

    fn print_with_path(&self, path: &[(usize, usize)]) {
        let mut grid = vec![vec!['.'; self.max_size]; self.max_size];

        for row in 0..self.max_size {
            for &col in &self.rows[row] {
                grid[row][col] = '#';
            }
        }

        for &(row, col) in path {
            if grid[row][col] == '.' {
                grid[row][col] = '*';
            }
        }

        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RobotResult {
    OutOfBounds,
    LoopDetected,
}

fn robot_path(path: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut covered_positions = Vec::new();

    for window in path.windows(2) {
        if let [start, end] = window {
            if start.0 == end.0 {
                // Horizontal movement
                let row = start.0;
                let range = if start.1 < end.1 {
                    start.1..=end.1
                } else {
                    end.1..=start.1
                };
                for col in range {
                    covered_positions.push((row, col));
                }
            } else if start.1 == end.1 {
                // Vertical movement
                let col = start.1;
                let range = if start.0 < end.0 {
                    start.0..=end.0
                } else {
                    end.0..=start.0
                };
                for row in range {
                    covered_positions.push((row, col));
                }
            }
        }
    }

    covered_positions
}

type GridZ<const N: usize> = [[(usize, usize); N]; N];

#[derive(Debug, Clone)]
struct Map<const N: usize> {
    grid: GridZ<N>,
    max_size: usize,
    start: (usize, usize),
}

impl<const N: usize> Map<N> {
    fn parse(input: &str) -> Self {
        let max_size = input.lines().count();
        let obstacles: Vec<_> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, ch)| ch == '#')
                    .map(move |(col, _)| (row, col))
            })
            .collect();

        let start = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .find(|&(_, ch)| ch == '^')
                    .map(move |(col, _)| (row, col))
            })
            .exactly_one()
            .unwrap();

        let mut rows = vec![Vec::<usize>::new(); N];
        let mut cols = vec![Vec::<usize>::new(); N];
        obstacles.iter().copied().for_each(|(r, c)| {
            rows[r].push(c);
            cols[c].push(r);
        });
        rows.iter_mut().for_each(|r| r.sort_unstable());
        cols.iter_mut().for_each(|c| c.sort_unstable());

        fn next_smaller(obstacles: &[usize], value: usize) -> usize {
            obstacles
                .iter()
                .copied()
                .filter(|&other| other < value)
                .last()
                .map(|other| other + 1)
                .unwrap_or(usize::MAX)
        }

        fn next_larger(obstacles: &[usize], value: usize) -> usize {
            obstacles
                .iter()
                .copied()
                .find(|&other| other > value)
                .map(|other| other - 1)
                .unwrap_or(usize::MAX)
        }

        // FIXME: Handle double and triple turns
        // FIXME: Handle adding obstacles
        let mut grid = [[(0usize, 0usize); N]; N];
        for (row, col) in obstacles.iter().copied() {
            grid[row][col] = (usize::MAX, usize::MAX);
            if row > 0 {
                // Hit from north, turning west
                let col_next = next_smaller(&rows[row - 1], col);
                if col_next != col {
                    grid[row - 1][col] = (row - 1, col_next);
                }
            }
            if row < max_size - 1 {
                // Hit from south, turning east
                let col_next = next_larger(&rows[row + 1], col);
                if col_next != col {
                    grid[row + 1][col] = (row + 1, col_next);
                }
            }
            if col > 0 {
                // Hit from west, turning south
                let row_next = next_larger(&cols[col - 1], row);
                if row_next != row {
                    grid[row][col - 1] = (row_next, col - 1);
                }
            }
            if col < max_size - 1 {
                // Hit from east, turning north
                let row_next = next_smaller(&cols[col + 1], row);
                if row_next != row {
                    grid[row][col + 1] = (row_next, col + 1);
                }
            }
        }

        let start_jump = next_smaller(&cols[start.1], start.0);
        grid[start.0][start.1] = (start_jump, start.1);

        Self {
            grid,
            max_size,
            start,
        }
    }

    fn simulate(&self) -> (RobotResult, Vec<(usize, usize)>) {
        let mut position = self.start;
        let mut direction = Direction::North;
        let mut visited_states: FxHashSet<(usize, usize, Direction)> = FxHashSet::default();
        let mut path = Vec::new();

        loop {
            path.push(position);

            if !visited_states.insert((position.0, position.1, direction)) {
                return (RobotResult::LoopDetected, path);
            }

            let (next_row, next_col) = self.grid[position.0][position.1];
            if next_row == usize::MAX || next_col == usize::MAX {
                // FIXME: Can't handle when we go out of bounds after double/triple turns
                // Add the last position before going out of bounds
                position = match direction {
                    Direction::North => (0, position.1),
                    Direction::South => (self.max_size - 1, position.1),
                    Direction::West => (position.0, 0),
                    Direction::East => (position.0, self.max_size - 1),
                };
                path.push(position);
                return (RobotResult::OutOfBounds, path);
            }

            if position.0 < next_row {
                direction = Direction::South;
            } else if position.0 > next_row {
                direction = Direction::North;
            } else if position.1 < next_col {
                direction = Direction::East;
            } else if position.1 > next_col {
                direction = Direction::West;
            } else {
                panic!(
                    "Invalid position {:?} -> {:?}",
                    position,
                    (next_row, next_col)
                );
            }

            position = (next_row, next_col);
        }
    }

    fn print(&self) {
        for row in self.grid.iter().take(self.max_size) {
            for (r, c) in row.iter().take(self.max_size).copied() {
                if r == usize::MAX && c == usize::MAX {
                    print!("(###) ");
                } else if r == 0 && c == 0 {
                    print!("(   ) ");
                } else {
                    let r = if r == usize::MAX { "X" } else { &r.to_string() };
                    let c = if c == usize::MAX { "X" } else { &c.to_string() };
                    print!("({},{}) ", r, c);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    const SAMPLE_2: &str = "\
....#.....
.....#...#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_build_graph() {
        let map = Map::<16>::parse(SAMPLE_2);
        map.print();
        let (result, path) = map.simulate();
        println!("{:?} {:?}", result, path);
        let steps = robot_path(&path);
        println!("{:?}", steps);
        println!("{:?}", steps.iter().unique().count());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE.into()).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE.into()).unwrap(), 6);
    }
}
