# Bddap Advent of Code Runner

Organize and run your rust advent of code solutions.

## Usage

src/year2015day1part1.rs

```rust
#[bddap_aoc::challenge(2015, 1, 1)]
fn run(_inp: &str) -> String {
	42.to_string()
}
```

src/year2015day1part3.rs

```rust
#[bddap_aoc::challenge(2015, 1, 2)]
fn run(_inp: &str) -> String {
	"Solution!".to_string()
}
```

src/year3243day2part1.rs

```rust
#[bddap_aoc::challenge(3243, 2, 1)]
fn run(_inp: &str) -> String {
	"Futrure solution!".to_string()
}
```

src/main.rs

```rust
mod day1part1;
mod day1part2;

fn main() {
	bddap_aoc::default_run();
}
```

```bash
cargo run -- login
> Enter session cookie from https://adventofcode.com/ : <cookie>
> Session cookie has been saved.

cargo run -- run
> year 2015 day 1 part 1:
> 42
>
> year 2015 day 1 part 2:
> Solution!
>
> year 3243 day 2 part 1:
> Challenge not yet released.

cargo run -- run --year 2015 --day 1 --part 2
> year 2015 day 1 part 2:
> Solution!
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
