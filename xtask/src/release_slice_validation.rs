use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

const RELEASE_MANIFEST: &str = "release/release-manifest.toml";
const VECTOR_PATH: &str = "validation/release_slice/m00_reference_vectors.tsv";
const SUMMARY_PATH: &str = "generated/release_slice_validation_summary.json";
const VECTOR_SCHEMA: &str = "aerocodex.release_slice_vector.v1";
const SUMMARY_SCHEMA: &str = "aerocodex.release_slice_validation_summary.v1";
const EXPECTED_HEADER: &str = "schema_version\tcase_id\tformula_id\tinput_a\tinput_b\tinput_c\texpected\tabs_tolerance\trel_tolerance\tulp_tolerance\tcontract\tvalidation_card\tsource_record\tpromotion_packet";

#[derive(Debug, Clone, PartialEq, Eq)]
struct VectorRecord {
    case_id: String,
    formula_id: String,
    contract: String,
    validation_card: String,
    source_record: String,
    promotion_packet: String,
}

pub fn verify_release_slice_validation(root: &Path) -> Result<(), String> {
    let manifest_text = read_utf8(root, RELEASE_MANIFEST)?;
    let formula_ids = release_formula_ids(&manifest_text)?;
    let vector_text = read_utf8(root, VECTOR_PATH)?;
    let vectors = parse_vectors(&vector_text)?;
    validate_vectors(root, &formula_ids, &vectors)?;

    let summary = render_summary(&formula_ids, &vectors);
    let checked = read_utf8(root, SUMMARY_PATH)?;
    if checked != summary {
        return Err(format!(
            "{SUMMARY_PATH} is stale; regenerate it from {RELEASE_MANIFEST} and {VECTOR_PATH}"
        ));
    }

    println!(
        "verified release-slice validation: formulas={}; analytical_vectors={}; linked_packets={}; generated_summary=fresh",
        formula_ids.len(),
        vectors.len(),
        formula_ids.len()
    );
    Ok(())
}

fn read_utf8(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))
}

fn release_formula_ids(text: &str) -> Result<Vec<String>, String> {
    let mut in_formula = false;
    let mut ids = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line == "[[formulas]]" {
            in_formula = true;
            continue;
        }
        if line.starts_with("[[") && line.ends_with("]]") {
            in_formula = false;
        }
        if in_formula {
            if let Some(value) = line
                .strip_prefix("identifier = \"")
                .and_then(|value| value.strip_suffix('"'))
            {
                ids.push(value.to_string());
                in_formula = false;
            }
        }
    }
    if ids.len() != 12 {
        return Err(format!(
            "{RELEASE_MANIFEST} must declare exactly 12 release formulas, found {}",
            ids.len()
        ));
    }
    let unique: BTreeSet<&str> = ids.iter().map(String::as_str).collect();
    if unique.len() != ids.len() {
        return Err(format!(
            "{RELEASE_MANIFEST} contains duplicate formula identifiers"
        ));
    }
    ids.sort();
    Ok(ids)
}

fn parse_vectors(text: &str) -> Result<Vec<VectorRecord>, String> {
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| format!("{VECTOR_PATH} is empty"))?;
    if header != EXPECTED_HEADER {
        return Err(format!("{VECTOR_PATH} has an unexpected header"));
    }

    let mut vectors = Vec::new();
    let mut case_ids = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 14 {
            return Err(format!(
                "{VECTOR_PATH} line {} has {} fields; expected 14",
                index + 2,
                fields.len()
            ));
        }
        if fields[0] != VECTOR_SCHEMA {
            return Err(format!(
                "{VECTOR_PATH} line {} has unsupported schema `{}`",
                index + 2,
                fields[0]
            ));
        }
        if fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "{VECTOR_PATH} line {} has an empty field",
                index + 2
            ));
        }
        for field_index in [3usize, 4, 5, 6, 7, 8] {
            fields[field_index].parse::<f64>().map_err(|error| {
                format!(
                    "{VECTOR_PATH} line {} field {} is not f64: {error}",
                    index + 2,
                    field_index + 1
                )
            })?;
        }
        fields[9].parse::<u64>().map_err(|error| {
            format!(
                "{VECTOR_PATH} line {} ULP tolerance is not u64: {error}",
                index + 2
            )
        })?;
        if !case_ids.insert(fields[1].to_string()) {
            return Err(format!("{VECTOR_PATH} repeats case_id `{}`", fields[1]));
        }
        vectors.push(VectorRecord {
            case_id: fields[1].to_string(),
            formula_id: fields[2].to_string(),
            contract: fields[10].to_string(),
            validation_card: fields[11].to_string(),
            source_record: fields[12].to_string(),
            promotion_packet: fields[13].to_string(),
        });
    }
    if vectors.is_empty() {
        return Err(format!("{VECTOR_PATH} contains no vector rows"));
    }
    Ok(vectors)
}

fn validate_vectors(
    root: &Path,
    formula_ids: &[String],
    vectors: &[VectorRecord],
) -> Result<(), String> {
    let expected: BTreeSet<&str> = formula_ids.iter().map(String::as_str).collect();
    let mut counts = BTreeMap::<&str, usize>::new();
    for vector in vectors {
        if !expected.contains(vector.formula_id.as_str()) {
            return Err(format!(
                "{} references non-release formula `{}`",
                vector.case_id, vector.formula_id
            ));
        }
        *counts.entry(vector.formula_id.as_str()).or_default() += 1;
        for relative in [
            &vector.contract,
            &vector.validation_card,
            &vector.source_record,
            &vector.promotion_packet,
        ] {
            let path = root.join(relative);
            if !path.is_file() {
                return Err(format!(
                    "{} references missing evidence file `{relative}`",
                    vector.case_id
                ));
            }
        }
        let packet = read_utf8(root, &vector.promotion_packet)?;
        for marker in [
            format!("formula_id: {}", vector.formula_id),
            "previous_status: research_required".to_string(),
            "requested_status: implementation_verified".to_string(),
            "requested_execution_policy: normal_research".to_string(),
            "decision: recommend_implementation_verified".to_string(),
            "non_claims:".to_string(),
        ] {
            if !packet.lines().any(|line| line.trim() == marker) {
                return Err(format!(
                    "{} is incomplete for `{}`; missing `{marker}`",
                    vector.promotion_packet, vector.formula_id
                ));
            }
        }
        for field in [
            "legacy_formula_id",
            "runtime_symbol",
            "equation_batch",
            "runtime_link",
            "cli_dispatch",
            "test_path",
            "commands",
            "traceability_review",
            "implementation_review",
        ] {
            let value = packet_field(&packet, field).ok_or_else(|| {
                format!(
                    "{} is incomplete for `{}`; missing `{field}`",
                    vector.promotion_packet, vector.formula_id
                )
            })?;
            if value.is_empty() {
                return Err(format!(
                    "{} field `{field}` is empty for `{}`",
                    vector.promotion_packet, vector.formula_id
                ));
            }
        }
        for field in [
            "equation_batch",
            "runtime_link",
            "cli_dispatch",
            "test_path",
        ] {
            let relative = packet_field(&packet, field).expect("required packet field");
            if !root.join(relative).is_file() {
                return Err(format!(
                    "{} field `{field}` references missing file `{relative}`",
                    vector.promotion_packet
                ));
            }
        }
        for marker in [
            "decision: accepted",
            "cargo test -p aero-codex-astrodynamics --test release_slice_validation",
            "cargo run -p xtask -- verify-release-slice-validation",
        ] {
            if !packet.contains(marker) {
                return Err(format!(
                    "{} is incomplete for `{}`; missing `{marker}`",
                    vector.promotion_packet, vector.formula_id
                ));
            }
        }
    }

    let covered: BTreeSet<&str> = counts.keys().copied().collect();
    if covered != expected {
        let missing: Vec<&str> = expected.difference(&covered).copied().collect();
        return Err(format!(
            "{VECTOR_PATH} does not cover every release formula; missing {}",
            missing.join(", ")
        ));
    }
    Ok(())
}

fn packet_field<'a>(packet: &'a str, field: &str) -> Option<&'a str> {
    let prefix = format!("{field}:");
    packet
        .lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}

fn render_summary(formula_ids: &[String], vectors: &[VectorRecord]) -> String {
    let mut counts = BTreeMap::<&str, usize>::new();
    for vector in vectors {
        *counts.entry(vector.formula_id.as_str()).or_default() += 1;
    }

    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"schema_version\": \"{SUMMARY_SCHEMA}\",\n"));
    out.push_str(&format!(
        "  \"release_manifest\": \"{RELEASE_MANIFEST}\",\n"
    ));
    out.push_str(&format!("  \"vector_source\": \"{VECTOR_PATH}\",\n"));
    out.push_str(&format!("  \"formula_count\": {},\n", formula_ids.len()));
    out.push_str(&format!(
        "  \"analytical_vector_count\": {},\n",
        vectors.len()
    ));
    out.push_str("  \"validation_scope\": \"release_slice_only\",\n");
    out.push_str("  \"claim_boundary\": \"implementation evidence for Research Software Alpha; not flight, mission, operational, regulatory, habitat, life-support, or certification evidence\",\n");
    out.push_str("  \"formulas\": [\n");
    for (index, formula_id) in formula_ids.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"formula_id\": \"{formula_id}\",\n"));
        out.push_str(&format!(
            "      \"analytical_vector_count\": {},\n",
            counts.get(formula_id.as_str()).copied().unwrap_or(0)
        ));
        out.push_str("      \"promotion_packet_state\": \"complete_recommendation\"\n");
        out.push_str(if index + 1 == formula_ids.len() {
            "    }\n"
        } else {
            "    },\n"
        });
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_formula_parser_requires_exact_unique_slice() {
        let mut manifest = String::new();
        for index in 0..12 {
            manifest.push_str(&format!(
                "[[formulas]]\nidentifier = \"m00.fixture.{index:02}\"\n"
            ));
        }
        let ids = release_formula_ids(&manifest).expect("twelve unique formulas");
        assert_eq!(ids.len(), 12);
        let duplicate = manifest.replace("m00.fixture.11", "m00.fixture.10");
        assert!(release_formula_ids(&duplicate).is_err());
    }

    #[test]
    fn vector_parser_rejects_duplicate_case_ids() {
        let row = "aerocodex.release_slice_vector.v1\tcase-1\tm00.fixture\t1\t1\t1\t1\t0\t0\t0\tcontract\tcard\tsource\tpacket\n";
        let text = format!("{EXPECTED_HEADER}\n{row}{row}");
        let error = parse_vectors(&text).expect_err("duplicate case must fail");
        assert!(error.contains("repeats case_id"));
    }

    #[test]
    fn summary_is_deterministic_and_release_scoped() {
        let ids = vec!["m00.a".to_string(), "m00.b".to_string()];
        let vectors = vec![
            VectorRecord {
                case_id: "a-1".to_string(),
                formula_id: "m00.a".to_string(),
                contract: "c".to_string(),
                validation_card: "v".to_string(),
                source_record: "s".to_string(),
                promotion_packet: "p".to_string(),
            },
            VectorRecord {
                case_id: "b-1".to_string(),
                formula_id: "m00.b".to_string(),
                contract: "c".to_string(),
                validation_card: "v".to_string(),
                source_record: "s".to_string(),
                promotion_packet: "p".to_string(),
            },
        ];
        let summary = render_summary(&ids, &vectors);
        assert!(summary.contains("\"validation_scope\": \"release_slice_only\""));
        assert_eq!(summary.matches("\"formula_id\"").count(), 2);
        assert_eq!(summary, render_summary(&ids, &vectors));
    }
}
