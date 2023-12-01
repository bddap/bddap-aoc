mod api;
mod challenge;
mod input_getter;
mod persist;

use clap::Parser;
use linkme::distributed_slice;

pub use bddap_aoc_macros::{register, unregistered_challenge};
pub use challenge::Challenge;
pub use input_getter::Getter;
pub use linkme;

#[distributed_slice]
pub static CHALLENGES: [Challenge] = [..];

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
enum Args {
    #[clap(about = "Run challenge[s]")]
    Run(Run),
    #[clap(about = "Save your session token so challenges inputs can be downloaded")]
    Login,
}

#[derive(clap::Parser)]
struct Run {
    /// Only run challenges for the given year[s]
    #[arg(short, long)]
    year: Vec<usize>,

    /// Only run challenges for the given day[s].
    #[arg(short, long)]
    day: Vec<usize>,

    /// Only run challenges for the given part[s].
    #[arg(short, long)]
    part: Vec<usize>,
}

/// Get settings from command line arguments.
pub fn run_default() {
    let argss = Args::parse();

    let run = match argss {
        Args::Run(run) => run,
        Args::Login => {
            unwrap_or_print(persist::login());
            return;
        }
    };

    let getter = unwrap_or_print(Getter::load());

    let mut todos: Vec<&Challenge> = CHALLENGES
        .iter()
        .filter(|c| run.year.contains(&c.year) || run.year.is_empty())
        .filter(|c| run.day.contains(&c.day) || run.day.is_empty())
        .filter(|c| run.part.contains(&c.part) || run.part.is_empty())
        .collect();

    if todos.is_empty() {
        eprintln!("No matching challenges.");
    }

    todos.sort_by_key(|c| (c.year, c.day, c.part));

    for challenge in todos {
        println!(
            "year {} day {} part {} - {}",
            challenge.year, challenge.day, challenge.part, challenge.name
        );
        let input = match getter.get_input(challenge.year, challenge.day) {
            Ok(input) => input,
            Err(api::Error::ChallengeNotReady { time_till_ready }) => {
                eprintln!(
                    "Challenge not ready yet. Try again in {}",
                    humantime::format_duration(time_till_ready)
                );
                continue;
            }
            Err(api::Error::Other(e)) => {
                eprintln!("Error getting input: {}", e);
                continue;
            }
        };
        let output = challenge.run(&input);
        println!("{}", output);
        println!();
    }
}

fn unwrap_or_print<T, E: std::fmt::Display>(result: Result<T, E>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
