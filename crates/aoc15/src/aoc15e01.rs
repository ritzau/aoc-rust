use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};

#[allow(clippy::result_large_err)]
pub fn not_quite_lisp(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "Not Quite Lisp");

    let input = input
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Failed to read the input for day {day}: {e}")))?;

    let floor_count = count_floors(&input);
    println!("aoc15e01a: {}", floor_count);

    let steps = find_basement(&input).unwrap();
    println!("aoc15e01b: {}", steps);

    Ok(floor_count == 232 && steps == 1783)
}

fn count_floors(input: impl AsRef<str>) -> i32 {
    input
        .as_ref()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            otherwise => panic!("Unkown char: {}", otherwise),
        })
        .sum()
}

fn find_basement(input: impl AsRef<str>) -> Option<usize> {
    let floors: Vec<i32> = input
        .as_ref()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => panic!("Unkown char: {}", c),
        })
        .scan(0, |state, x| {
            if *state == -1 {
                return None;
            }
            *state += x;
            Some(*state)
        })
        .collect();

    if floors.last() == Some(&-1) {
        Some(floors.len())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_count_floors() {
        assert_eq!(count_floors(""), 0);
        assert_eq!(count_floors("("), 1);
        assert_eq!(count_floors(")"), -1);
        assert_eq!(count_floors("()"), 0);
        assert_eq!(count_floors(")("), 0);
    }

    #[test]
    fn can_find_basement() {
        assert_eq!(find_basement(")"), Some(1));
        assert_eq!(find_basement("(()"), None);
    }
}