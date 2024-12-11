use aoc::{AocCache, PuzzleError, PuzzleResult, Year};

mod e00;
mod e01;
mod e02;
mod e03;
mod e04;
mod e05;
mod e06;
mod e06b;
mod e07;
mod e08;
mod e09;
mod e10;
mod e11;
mod e12;
mod e13;
mod e14;
mod e15;
mod e16;
mod e17;
mod e18;

const YEAR: Year = Year(2024);

type AoCSolution = fn(&AocCache) -> PuzzleResult<()>;

pub fn solve() -> PuzzleResult<()> {
    run(&[
        e01::solve,
        e02::solve,
        e03::solve,
        e04::solve,
        e05::solve,
        e06::solve,
        e06b::solve,
        e07::solve,
        e08::solve,
        e09::solve,
        e10::solve,
        e11::solve,
        e12::solve,
        e13::solve,
        e14::solve,
        e15::solve,
        e16::solve,
        e17::solve,
        e18::solve,
    ])
}

fn run(seq: &[AoCSolution]) -> PuzzleResult<()> {
    for &f in seq {
        verify(f)?;
    }

    Ok(())
}

fn verify(f: AoCSolution) -> PuzzleResult<()> {
    let cache = AocCache::default();

    let start = std::time::Instant::now();

    let result = match f(&cache) {
        Err(err) => Err(PuzzleError::Solution(format!(
            "Execution failed: {:?}",
            err
        ))),
        _ => Ok(()),
    };

    println!("Duration: {:.1?}", start.elapsed());

    result
}
