use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::{self, create_dir_all, rename, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

const EXCLUDE_SLOW_SOLUTIONS: bool = true;

pub mod aoc15e01;
pub mod aoc15e02;
pub mod aoc15e03;
pub mod aoc15e04;
pub mod aoc15e05;
pub mod aoc15e06;

pub use aoc15e01::not_quite_lisp as e01_not_quite_lisp;
pub use aoc15e02::i_was_told_there_would_be_no_math as e02_i_was_told_there_would_be_no_math;
pub use aoc15e03::perfectly_spherical_houses_in_a_vacuum as e03_perfectly_spherical_houses_in_a_vacuum;
pub use aoc15e04::the_ideal_stocking_stuffer as e04_the_ideal_stocking_stuffer;
pub use aoc15e05::doesnt_he_have_intern_elves_for_this as e05_doesnt_he_have_intern_elves_for_this;
pub use aoc15e06::probably_a_fire_hazard as e06_probably_a_fire_hazard;

type PuzzleResult<T> = Result<T, PuzzleError>;
type AoCSolution = fn(u8, Box<dyn PuzzleInput>) -> PuzzleResult<bool>;

#[derive(Debug)]
pub enum PuzzleError {
    Input(String),
    Verification(String),
    Solution(String, Box<dyn Error>),
    DownloadFailed(String, Box<dyn Error>),
    Cache(String, Box<dyn Error>),
    Processing(String, Box<dyn Error>),
}

impl Error for PuzzleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn run<T>(seq: T) -> PuzzleResult<()>
where
    T: IntoIterator<Item = AoCSolution>,
{
    for (day, f) in seq.into_iter().enumerate() {
        let day = (1 + day).try_into().unwrap();
        verify(day, f)?;
    }

    Ok(())
}

fn verify(day: u8, f: AoCSolution) -> PuzzleResult<()> {
    let cache = PuzzleCache::default();
    let input = cache.get_input(2015, day).map_err(|e| {
        PuzzleError::Input(format!("Failed to get input for 2015 day {day}: {e:?}"))
    })?;

    match f(day, input) {
        Ok(false) => Err(PuzzleError::Verification(format!(
            "Verification for day {day} failed"
        ))),
        Err(err) => Err(PuzzleError::Solution(
            format!("Execution of day {day} failed: {:?}", err),
            err.into(),
        )),
        _ => Ok(()),
    }
}

fn header(day: u8, title: impl AsRef<str>) {
    println!();
    println!("-- Day {}: {} ---", day, title.as_ref())
}

pub trait PuzzleInput {
    fn input(&self) -> Result<BufReader<Box<dyn Read>>, PuzzleError>;

    fn read_to_string(&self) -> Result<String, Box<dyn Error>> {
        let mut reader = self.input()?; // Get the reader from the input
        let mut content = String::new();
        reader.read_to_string(&mut content)?; // Read all content to the string
        Ok(content)
    }

    fn lines(&self) -> PuzzleResult<Box<dyn Iterator<Item = PuzzleResult<String>>>> {
        let iterator = self.input()?.lines().map(|line| {
            line.map_err(|e| {
                PuzzleError::Processing(format!("Failed to read a line: {e}"), e.into())
            })
        });

        Ok(Box::new(iterator))
    }
}

#[derive(Debug)]
pub struct PuzzleFileInput {
    path: PathBuf,
}

impl PuzzleInput for PuzzleFileInput {
    fn input(&self) -> PuzzleResult<BufReader<Box<dyn Read>>> {
        let file = File::open(&self.path).map_err(|e| {
            PuzzleError::Input(format!(
                "Failed to open file at {}: {}",
                self.path.display(),
                e
            ))
        })?;

        Ok(BufReader::new(Box::new(file)))
    }
}

impl PuzzleFileInput {
    fn new(path: PathBuf) -> PuzzleFileInput {
        PuzzleFileInput { path }
    }
}

#[derive(Debug)]
pub struct PuzzleCache {
    root: PathBuf,
}

impl Default for PuzzleCache {
    fn default() -> Self {
        Self {
            root: PathBuf::from("cache"),
        }
    }
}

impl PuzzleCache {
    fn get_session(&self) -> String {
        let path = self.root.join("session.txt");
        fs::read_to_string(path)
            .expect("Session file not found")
            .trim()
            .to_string()
    }

    pub fn get_input(&self, year: u16, day: u8) -> PuzzleResult<Box<dyn PuzzleInput>> {
        let file_path = self.path(year, day);
        let tmp_file_path = format!("{}.tmp", file_path.display());

        // Check if the file already exists, return the stream from the file if it does
        if file_path.is_file() {
            return Ok(Box::new(PuzzleFileInput::new(file_path)) as Box<dyn PuzzleInput>);
        }

        // If file doesn't exist, download it to the .tmp file
        println!("File not found, downloading input.");

        let session = self.get_session();

        if let Some(parent) = PathBuf::from(&tmp_file_path).parent() {
            create_dir_all(parent).map_err(|e| {
                PuzzleError::Cache(
                    format!("Failed to create cache directory {}: {e}", parent.display()),
                    e.into(),
                )
            })?;
        }

        // Open the .tmp file for writing
        let mut tmp_file = File::create(&tmp_file_path).map_err(|e| {
            PuzzleError::Cache(
                format!("Failed to open file at {}: {}", tmp_file_path, e),
                e.into(),
            )
        })?;

        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        // Fetch input via streaming
        let response = ureq::get(&url)
            .set("Cookie", &format!("session={}", session))
            .call()
            .map_err(|e| {
                PuzzleError::DownloadFailed(format!("Failed to download {url}: {e}"), e.into())
            })?;

        // Stream the response into the .tmp file
        let mut reader = response.into_reader();
        let mut buffer = [0; 8192]; // 8 KB chunks

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break; // EOF reached
            }
            tmp_file.write_all(&buffer[..bytes_read]).map_err(|e| {
                PuzzleError::Cache(
                    format!("Can't write to file {}: {e}", tmp_file_path),
                    e.into(),
                )
            })?;
        }

        // Rename the .tmp file to the final file name (this is atomic on most filesystems)
        rename(&tmp_file_path, &file_path).map_err(|e| {
            PuzzleError::Cache(
                format!(
                    "Can't rename {} to {}: {e}",
                    tmp_file_path,
                    file_path.display()
                ),
                e.into(),
            )
        })?;

        Ok(Box::new(PuzzleFileInput::new(file_path)))
    }

    fn path(&self, year: u16, day: u8) -> PathBuf {
        self.root
            .join("aoc15")
            .join(format!("{}_{}.txt", year, day))
    }
}