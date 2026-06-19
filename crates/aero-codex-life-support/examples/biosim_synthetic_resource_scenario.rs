use aero_codex_life_support::biosim_process::{run_biosim_scenario, BioSimProcess};
use aero_codex_life_support::biosim_scenario::{
    BioSimCompartment, BioSimCompartmentId, BioSimInitialStore, BioSimProcessId, BioSimScenario,
    BioSimScenarioClock, BioSimScenarioMetadata, BioSimScenarioResourceKind,
};
use aero_codex_life_support::biosim_scenario_report::{
    build_biosim_scenario_resource_ledger, format_biosim_scenario_friend_test_report,
};

fn compartment(raw: &str) -> BioSimCompartmentId {
    BioSimCompartmentId::new(raw).expect("fixed synthetic compartment id")
}

fn process_id(raw: &str) -> BioSimProcessId {
    BioSimProcessId::new(raw).expect("fixed synthetic process id")
}

fn fixed_synthetic_scenario() -> BioSimScenario {
    let crew = compartment("crew_cabin");
    let buffer = compartment("buffer_store");
    BioSimScenario::new(
        BioSimScenarioMetadata::new(
            "synthetic_b2c_example",
            "Synthetic B2c example",
            "research_required",
        )
        .expect("fixed metadata"),
        vec![
            BioSimCompartment::new("crew_cabin", "Crew cabin").expect("fixed compartment"),
            BioSimCompartment::new("buffer_store", "Buffer store").expect("fixed compartment"),
        ],
        vec![
            BioSimInitialStore::new(crew, BioSimScenarioResourceKind::Oxygen, 1.0),
            BioSimInitialStore::new(buffer, BioSimScenarioResourceKind::Oxygen, 5.0),
        ],
        BioSimScenarioClock::new(10.0, 2),
    )
}

fn fixed_synthetic_processes() -> Vec<BioSimProcess> {
    vec![BioSimProcess::transfer(
        process_id("oxygen_transfer"),
        compartment("buffer_store"),
        compartment("crew_cabin"),
        BioSimScenarioResourceKind::Oxygen,
        0.1,
    )
    .expect("fixed clean-room transfer process")]
}

fn main() {
    let scenario = fixed_synthetic_scenario();
    let processes = fixed_synthetic_processes();
    let replay = run_biosim_scenario(&scenario, &processes).expect("fixed B2b-2 replay");
    let report = build_biosim_scenario_resource_ledger(&replay).expect("fixed B2c ledger report");
    print!("{}", format_biosim_scenario_friend_test_report(&report));
}
