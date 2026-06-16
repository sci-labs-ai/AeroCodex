//! BioSim-RS clean-room friend-test smoke example.
//!
//! Run with:
//! `cargo run -p aero-codex-life-support --example biosim_friend_test_smoke`
//!
//! The printed report is a static API/example-output smoke artifact only with
//! `research_required` validation status. It does not execute BioSim scenarios,
//! load external fixtures, or make readiness claims.

use aero_codex_life_support::biosim_resource_tick::{
    format_biosim_friend_test_report, run_biosim_cli_api_smoke_report,
};

fn main() {
    let report = run_biosim_cli_api_smoke_report()
        .expect("built-in BioSim-RS friend-test smoke report should be generated");
    print!("{}", format_biosim_friend_test_report(&report.value));
}
