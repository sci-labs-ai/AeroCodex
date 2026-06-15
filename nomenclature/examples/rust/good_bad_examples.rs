//! AeroCodex nomenclature examples.
//! These are illustrative snippets, not a complete crate.

// BAD: compact math notation leaked into durable Rust code.
#[allow(dead_code)]
fn bad_velocity_estimate(x: &[f64], dt: f64) -> f64 {
    let n = x.len();
    (x[n - 1] - x[0]) / ((n - 1) as f64 * dt)
}

// GOOD: semantic Rust identifiers that bridge to the equation symbol table.
#[allow(dead_code)]
fn estimate_window_velocity_mps(
    position_samples_meters: &[f64],
    sample_interval_seconds: f64,
) -> Result<f64, &'static str> {
    let sample_count = position_samples_meters.len();

    if sample_count < 2 {
        return Err("at least two samples are required");
    }

    if sample_interval_seconds <= 0.0 {
        return Err("sample interval must be positive");
    }

    // Implements E-trajectory-fit-014.
    // Bridge: acx:bridge:trajectory.fit.window:E014:v1
    let elapsed_seconds = (sample_count - 1) as f64 * sample_interval_seconds;
    let estimated_velocity_mps =
        (position_samples_meters[sample_count - 1] - position_samples_meters[0]) / elapsed_seconds;

    Ok(estimated_velocity_mps)
}

// BAD: const generics N/M are not self-describing.
#[allow(dead_code)]
struct BadMatrix<T, const N: usize, const M: usize> {
    data: [[T; M]; N],
}

// GOOD: const generics describe their semantic role.
#[allow(dead_code)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

// GOOD: tiny local loop exception. In core logic, prefer semantic names.
#[allow(dead_code)]
fn tiny_loop_ok() -> usize {
    let mut total = 0;
    for i in 0..3 {
        total += i;
    }
    total
}

// BAD: source term leaks into canonical Rust identifier.
#[allow(dead_code)]
struct BadAircraftRecord {
    tail_number: String,
}

// GOOD: canonical AeroCodex term.
#[allow(dead_code)]
struct AircraftRecord {
    aircraft_registration: String,
}
