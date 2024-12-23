# Advent of Code Rust Solutions

![Actions Status](https://github.com/ritzau/aoc-rust/actions/workflows/rust.yml/badge.svg)

This repository contains solutions for the [Advent of Code](https://adventofcode.com/) challenges, implemented in Rust.

## Setup and Running

1. **Clone the repository**:
   ```sh
   git clone https://github.com/ritzau/aoc-rust.git
   cd aoc-rust
   ```

2. **Prepare the session file**:
    - Copy your session cookie from the browser and paste it into `cache/session.txt`.
    - Or add the files manually to the cache named: `cache/aoc/2024/01.txt ...`

3. **Run**:
    - Run the project (defaulting to s24):
      ```sh
      cargo run --release
      ```
    - Run the s15:
      ```sh
      cargo run -p aoc15 --release
      ```
