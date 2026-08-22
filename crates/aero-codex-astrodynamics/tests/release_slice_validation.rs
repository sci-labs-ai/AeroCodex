use aero_codex_astrodynamics::{
    m00_canonical_mu_from_units, m00_canonical_speed_unit_from_du_tu,
    m00_canonical_speed_unit_from_mu_du, m00_canonical_time_unit_from_mu_du,
    m00_degrees_to_radians, m00_distance_from_canonical, m00_distance_to_canonical,
    m00_radians_to_degrees, m00_speed_from_canonical, m00_speed_to_canonical,
    m00_time_from_canonical, m00_time_to_canonical,
};
use std::{collections::BTreeSet, path::Path};

const VECTORS: &str = include_str!("../../../validation/release_slice/m00_reference_vectors.tsv");
const EXPECTED_HEADER: &str = "schema_version\tcase_id\tformula_id\tinput_a\tinput_b\tinput_c\texpected\tabs_tolerance\trel_tolerance\tulp_tolerance\tcontract\tvalidation_card\tsource_record\tpromotion_packet";

#[derive(Debug)]
struct Vector<'a> {
    case_id: &'a str,
    formula_id: &'a str,
    input: [f64; 3],
    expected: f64,
    absolute: f64,
    relative: f64,
    ulps: u64,
    evidence: [&'a str; 4],
}

fn vectors() -> Vec<Vector<'static>> {
    let mut lines = VECTORS.lines();
    assert_eq!(lines.next(), Some(EXPECTED_HEADER));
    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            assert_eq!(fields.len(), 14, "malformed release vector: {line}");
            assert_eq!(fields[0], "aerocodex.release_slice_vector.v1");
            Vector {
                case_id: fields[1],
                formula_id: fields[2],
                input: [
                    fields[3].parse().unwrap(),
                    fields[4].parse().unwrap(),
                    fields[5].parse().unwrap(),
                ],
                expected: fields[6].parse().unwrap(),
                absolute: fields[7].parse().unwrap(),
                relative: fields[8].parse().unwrap(),
                ulps: fields[9].parse().unwrap(),
                evidence: [fields[10], fields[11], fields[12], fields[13]],
            }
        })
        .collect()
}

fn evaluate(formula_id: &str, input: [f64; 3]) -> f64 {
    match formula_id {
        "m00.canonical.time_unit_from_mu_du" => {
            m00_canonical_time_unit_from_mu_du(input[0], input[1]).unwrap()
        }
        "m00.canonical.speed_unit_from_du_tu" => {
            m00_canonical_speed_unit_from_du_tu(input[0], input[1]).unwrap()
        }
        "m00.canonical.speed_unit_from_mu_du" => {
            m00_canonical_speed_unit_from_mu_du(input[0], input[1]).unwrap()
        }
        "m00.canonical.mu_from_units" => {
            m00_canonical_mu_from_units(input[0], input[1], input[2]).unwrap()
        }
        "m00.canonical.distance_to_canonical" => {
            m00_distance_to_canonical(input[0], input[1]).unwrap()
        }
        "m00.canonical.distance_from_canonical" => {
            m00_distance_from_canonical(input[0], input[1]).unwrap()
        }
        "m00.canonical.time_to_canonical" => {
            m00_time_to_canonical(input[0], input[1]).unwrap()
        }
        "m00.canonical.time_from_canonical" => {
            m00_time_from_canonical(input[0], input[1]).unwrap()
        }
        "m00.canonical.speed_to_canonical" => {
            m00_speed_to_canonical(input[0], input[1], input[2]).unwrap()
        }
        "m00.canonical.speed_from_canonical" => {
            m00_speed_from_canonical(input[0], input[1], input[2]).unwrap()
        }
        "m00.angle.deg_to_rad" => m00_degrees_to_radians(input[0]).unwrap(),
        "m00.angle.rad_to_deg" => m00_radians_to_degrees(input[0]).unwrap(),
        other => panic!("unsupported release formula {other}"),
    }
}

fn assert_policy(actual: f64, expected: f64, absolute: f64, relative: f64, ulps: u64) {
    assert!(actual.is_finite(), "actual result was not finite: {actual}");
    if actual == expected {
        return;
    }
    let difference = (actual - expected).abs();
    let relative_bound = relative * actual.abs().max(expected.abs());
    let ulp_distance = if actual.is_sign_negative() == expected.is_sign_negative() {
        actual.to_bits().abs_diff(expected.to_bits())
    } else {
        u64::MAX
    };
    assert!(
        difference <= absolute || difference <= relative_bound || ulp_distance <= ulps,
        "actual={actual:?}, expected={expected:?}, difference={difference}, absolute={absolute}, relative_bound={relative_bound}, ulp_distance={ulp_distance}, ulps={ulps}"
    );
}

#[test]
fn every_release_formula_matches_an_independently_linked_analytical_vector() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let vectors = vectors();
    let mut formula_ids = BTreeSet::new();
    let mut case_ids = BTreeSet::new();
    for vector in &vectors {
        assert!(case_ids.insert(vector.case_id), "duplicate {}", vector.case_id);
        formula_ids.insert(vector.formula_id);
        for evidence in vector.evidence {
            assert!(root.join(evidence).is_file(), "missing evidence {evidence}");
        }
        assert_policy(
            evaluate(vector.formula_id, vector.input),
            vector.expected,
            vector.absolute,
            vector.relative,
            vector.ulps,
        );
    }
    assert_eq!(vectors.len(), 12);
    assert_eq!(formula_ids.len(), 12);
}

#[test]
fn distance_time_speed_and_angle_round_trips_cover_signed_domain() {
    let values = [-1.0e9, -12.5, -0.0, 0.0, 1.0e-12, 3.5, 1.0e9];
    let scales = [1.0e-6, 1.0, 2.5, 1.0e6];
    for value in values {
        for scale in scales {
            let distance = m00_distance_from_canonical(
                m00_distance_to_canonical(value, scale).unwrap(),
                scale,
            )
            .unwrap();
            assert_policy(distance, value, 1.0e-12, 1.0e-12, 8);
            let time = m00_time_from_canonical(
                m00_time_to_canonical(value, scale).unwrap(),
                scale,
            )
            .unwrap();
            assert_policy(time, value, 1.0e-12, 1.0e-12, 8);
        }
        for distance_unit in scales {
            for time_unit in scales {
                let speed = m00_speed_from_canonical(
                    m00_speed_to_canonical(value, distance_unit, time_unit).unwrap(),
                    distance_unit,
                    time_unit,
                )
                .unwrap();
                assert_policy(speed, value, 1.0e-12, 1.0e-12, 8);
            }
        }
        let degrees = m00_radians_to_degrees(m00_degrees_to_radians(value).unwrap()).unwrap();
        assert_policy(degrees, value, 1.0e-12, 1.0e-12, 8);
    }
}

#[test]
fn canonical_scale_one_is_identity() {
    for value in [-f64::MAX / 4.0, -1.0, -0.0, 0.0, 1.0, f64::MAX / 4.0] {
        assert_eq!(m00_distance_to_canonical(value, 1.0).unwrap(), value);
        assert_eq!(m00_distance_from_canonical(value, 1.0).unwrap(), value);
        assert_eq!(m00_time_to_canonical(value, 1.0).unwrap(), value);
        assert_eq!(m00_time_from_canonical(value, 1.0).unwrap(), value);
        assert_eq!(m00_speed_to_canonical(value, 1.0, 1.0).unwrap(), value);
        assert_eq!(m00_speed_from_canonical(value, 1.0, 1.0).unwrap(), value);
    }
}

#[test]
fn invalid_scales_nonfinite_values_and_overflow_fail_closed() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY, -f64::INFINITY] {
        assert!(m00_distance_to_canonical(1.0, invalid).is_err());
        assert!(m00_time_from_canonical(1.0, invalid).is_err());
        assert!(m00_speed_to_canonical(1.0, invalid, 1.0).is_err());
        assert!(m00_speed_from_canonical(1.0, 1.0, invalid).is_err());
    }
    for invalid in [f64::NAN, f64::INFINITY, -f64::INFINITY] {
        assert!(m00_degrees_to_radians(invalid).is_err());
        assert!(m00_radians_to_degrees(invalid).is_err());
        assert!(m00_distance_to_canonical(invalid, 1.0).is_err());
    }
    assert!(m00_distance_from_canonical(f64::MAX, 2.0).is_err());
    assert!(m00_time_from_canonical(f64::MAX, 2.0).is_err());
    assert!(m00_speed_from_canonical(f64::MAX, 2.0, 1.0).is_err());
    assert!(m00_canonical_time_unit_from_mu_du(1.0, f64::MAX).is_err());
    assert!(m00_canonical_mu_from_units(f64::MAX, 1.0, f64::MAX).is_err());
}

#[test]
fn broad_domain_loops_are_deterministic_without_random_seeds() {
    for exponent in -12..=12 {
        let scale = 10.0_f64.powi(exponent);
        for whole in -64..=64 {
            let value = f64::from(whole) * 0.375 * scale;
            let first = m00_distance_to_canonical(value, scale).unwrap();
            let second = m00_distance_to_canonical(value, scale).unwrap();
            assert_eq!(first.to_bits(), second.to_bits());
            assert_policy(first, f64::from(whole) * 0.375, 1.0e-12, 1.0e-12, 8);
        }
    }
}

#[test]
fn cross_platform_edge_vectors_preserve_binary64_contract() {
    let smallest = f64::from_bits(1);
    assert_eq!(m00_distance_to_canonical(smallest, 1.0).unwrap().to_bits(), 1);
    assert_eq!(m00_time_from_canonical(-0.0, 1.0).unwrap().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(m00_degrees_to_radians(180.0).unwrap().to_bits(), std::f64::consts::PI.to_bits());
    assert_eq!(m00_radians_to_degrees(std::f64::consts::PI).unwrap(), 180.0);
}
