#[bddap_aoc::register(2015, 1, 1)]
pub fn runa(_inp: &str) -> String {
    42.to_string()
}

#[bddap_aoc::register(2015, 1, 2)]
fn runb(_inp: &str) -> String {
    "Solution!".to_string()
}

#[bddap_aoc::unregistered_challenge(3243, 2, 1)]
fn runc(_inp: &str) -> String {
    "Futrure solution!".to_string()
}

fn main() {
    bddap_aoc::run_default();
}
