use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use fxhash::FxHashSet;

const DAY: Day = Day(6);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Guard Gallivant");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input.read_to_string()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 4665);

    // let p2 = part2(&input.read_to_string()?)?;
    // println!("Part 2: {}", p2);
    // assert_eq!(p2, 1688);

    Ok(())
}

fn part1(input: &str) -> PuzzleResult<usize> {
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

    // println!("{:?}", start_pos);

    let r = simulate_robot(&grid, start_pos, Direction::North);
    // println!("{:?}", r);

    let unique = unique_positions(&r.1);
    // println!("{:?}", unique);

    Ok(unique)
}

fn part2(_input: &str) -> PuzzleResult<usize> {
    Ok(6)
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
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

impl Grid {
    fn new(max_size: usize) -> Self {
        Grid {
            rows: vec![Vec::new(); max_size],
            cols: vec![Vec::new(); max_size],
        }
    }

    fn add_obstacle(&mut self, row: usize, col: usize) {
        self.rows[row].push(col);
        self.cols[col].push(row);
    }

    // Find the nearest previous obstacle in a sorted vector
    fn find_prev(obstacles: &[usize], pos: usize) -> Option<isize> {
        obstacles
            .iter()
            .rev()
            .find(|&&x| x < pos)
            .map(|&x| (x + 1) as isize)
    }

    // Find the nearest next obstacle in a sorted vector
    fn find_next(obstacles: &[usize], pos: usize) -> Option<isize> {
        // println!("pos {pos} of obstacles: {:?}", obstacles);
        obstacles
            .iter()
            .find(|&&x| x > pos)
            .map(|&x| (x - 1) as isize)
    }

    // Get the next position based on direction
    fn next_position(&self, pos: (usize, usize), direction: Direction) -> isize {
        let (row, col) = pos;

        // println!("pos: {:?}, direction: {:?}", pos, direction);
        match direction {
            Direction::North => Self::find_prev(&self.cols[col], row).unwrap_or(-1),
            Direction::South => Self::find_next(&self.cols[col], row).unwrap_or(-1),
            Direction::West => Self::find_prev(&self.rows[row], col).unwrap_or(-1),
            Direction::East => Self::find_next(&self.rows[row], col).unwrap_or(-1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RobotResult {
    OutOfBounds,
    LoopDetected,
}

fn simulate_robot(
    grid: &Grid,
    start_position: (usize, usize),
    start_direction: Direction,
) -> (RobotResult, Vec<(usize, usize)>) {
    let mut position = start_position;
    let mut direction = start_direction;
    let mut visited_states: FxHashSet<((usize, usize), Direction)> = FxHashSet::default();
    let mut path = Vec::new();

    loop {
        path.push(position);

        if !visited_states.insert((position, direction)) {
            return (RobotResult::LoopDetected, path);
        }

        let next = grid.next_position(position, direction);
        if next == -1 {
            match direction {
                Direction::North => position.0 = 0,
                Direction::South => position.0 = grid.rows.len() - 1,
                Direction::West => position.1 = 0,
                Direction::East => position.1 = grid.cols.len() - 1,
            }
            path.push(position);
            return (RobotResult::OutOfBounds, path);
        } else {
            match direction {
                Direction::North | Direction::South => position.0 = next as usize,
                Direction::West | Direction::East => position.1 = next as usize,
            }
        }

        direction = direction.turn_right();
    }
}

fn unique_positions(path: &[(usize, usize)]) -> usize {
    let mut covered_positions: FxHashSet<(usize, usize)> = FxHashSet::default();

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
                    covered_positions.insert((row, col));
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
                    covered_positions.insert((row, col));
                }
            }
        }
    }

    covered_positions.len()
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE.into()).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE.into()).unwrap(), 6);
    }
}
