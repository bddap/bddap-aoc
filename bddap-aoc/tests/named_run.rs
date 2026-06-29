//! Regression test for issue #7: a user challenge fn named `run` must not
//! collide with the wrapper fn the `register`/`unregistered_challenge` macros
//! generate (previously both were `fn run`, yielding E0428).

#[bddap_aoc::unregistered_challenge(2020, 1, 1)]
fn run(input: &str) -> String {
    format!("got: {input}")
}

#[test]
fn challenge_fn_named_run_compiles_and_works() {
    assert_eq!(run.name, "run");
    assert_eq!(run.run("hi"), "got: hi");
}
