#![forbid(unsafe_code)]

mod equation_batch;

use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

const REQUIRED_CARD_FIELDS: &[&str] = &[
    "id",
    "name",
    "category",
    "status",
    "source",
    "assumptions",
    "domain",
    "inputs",
    "outputs",
    "tests",
    "failure_modes",
    "notes",
];

const REQUIRED_CARD_LIST_FIELDS: &[&str] =
    &["assumptions", "inputs", "outputs", "tests", "failure_modes"];

const REQUIRED_SOURCE_REGISTRY_FIELDS: &[&str] = &[
    "id",
    "title",
    "category",
    "status",
    "intended_use",
    "requested_details",
    "implementation_notes",
    "limits",
    "notes",
];

const REQUIRED_SOURCE_REGISTRY_LIST_FIELDS: &[&str] = &[
    "intended_use",
    "requested_details",
    "implementation_notes",
    "limits",
];

fn required_formula_vault_top_level_fields() -> &'static [&'static str] {
    &[
        "schema_version",
        "record_status",
        "slice",
        "sources",
        "formula_contract",
        "validation_records",
        "evidence_plan",
        "promotion_gate",
        "non_claims",
    ]
}

fn required_formula_vault_slice_fields() -> &'static [&'static str] {
    &[
        "id",
        "title",
        "lifecycle_label",
        "stage4_chunk",
        "public_api_surface",
    ]
}

fn required_formula_vault_sources_fields() -> &'static [&'static str] {
    &["source_registry_seed_id", "validation_card_id"]
}

fn required_formula_vault_contract_list_fields() -> &'static [&'static str] {
    &[
        "formula_ids",
        "variables",
        "units",
        "coordinate_frames",
        "time_scales",
        "sign_conventions",
        "valid_domain",
        "singularities",
        "invalid_regions",
        "branch_behavior",
    ]
}

fn required_formula_vault_validation_fields() -> &'static [&'static str] {
    &[
        "required_source_registry_seed",
        "required_validation_card",
        "status",
        "status_upgrade_policy",
    ]
}

fn required_formula_vault_non_claims_true() -> &'static [&'static str] {
    &[
        "no_certification_evidence",
        "no_flight_readiness",
        "no_mission_readiness",
        "no_operational_approval",
        "no_regulated_use_approval",
        "no_bulk_m07_import",
        "no_external_parity_claim_without_evidence",
    ]
}

const ALLOWED_STATUSES: &[&str] = &[
    "research_required",
    "equation_traceable",
    "implementation_verified",
    "reference_validated",
    "experiment_validated",
];

const ALLOWED_CARD_CATEGORIES: &[&str] = &[
    "core",
    "constants",
    "atmosphere",
    "thermodynamics",
    "gas_dynamics",
    "aerodynamics",
    "propulsion",
    "heat_transfer",
    "structures",
    "flight_dynamics",
    "astrodynamics",
    "life_support",
    "validation",
];

const ALLOWED_SOURCE_CATEGORIES: &[&str] = &[
    "constants",
    "atmosphere",
    "thermodynamics",
    "thermodynamics_propulsion",
    "gas_dynamics",
    "aerodynamics",
    "propulsion",
    "heat_transfer",
    "structures",
    "flight_dynamics",
    "astrodynamics",
    "life_support",
    "validation",
];

const REQUIRED_SCHEMA_MARKERS: &[&str] = &[
    "\"$schema\"",
    "draft/2020-12",
    "\"additionalProperties\": false",
    "\"id\"",
    "\"name\"",
    "\"category\"",
    "\"status\"",
    "\"source\"",
    "\"assumptions\"",
    "\"domain\"",
    "\"inputs\"",
    "\"outputs\"",
    "\"tests\"",
    "\"failure_modes\"",
    "\"notes\"",
];

const FORBIDDEN_DEPENDENCY_TOKENS: &[&str] = &[
    "bindgen",
    "cc",
    "cmake",
    "pkg-config",
    "vcpkg",
    "coolprop",
    "refprop",
    "cantera",
    "blas",
    "lapack",
    "script-runtime",
    "matlab",
    "julia",
];

const APPROVED_PUBLIC_WORDING: &str = "AeroCodex is intended to become professional-grade, traceable aerospace research software suitable for academic, laboratory, and agency evaluation. It is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.";

const PUBLIC_WORDING_GUARDRAILS_PATH: &str = "docs/assurance/public_wording_guardrails.md";

const REQUIRED_PUBLIC_FORBIDDEN_CLAIM_PHRASES: &[&str] = &[
    "nasa-ready",
    "flight-ready",
    "mission-ready",
    "certified for flight",
    "operationally approved",
    "habitat-safe",
    "medical/life-support certified",
    "regulatory-approved",
];

const PUBLIC_WORDING_SCAN_PATHS: &[&str] = &[
    "README.md",
    "docs",
    "formula-vault/README.md",
    "validation/README.md",
];

const FORBIDDEN_READINESS_MARKERS: &[&str] = &[
    "certified: true",
    "flight_ready: true",
    "mission_ready: true",
    "operationally_approved: true",
    "approved_for_operations: true",
    "status: certified",
    "status: flight_ready",
    "status: mission_ready",
    "nasa-ready",
    "nasa_ready",
    "nasa ready",
    "nasa-approved",
    "nasa_approved",
    "nasa approved",
    "flight-ready",
    "flight_ready",
    "flight ready",
    "flight-readiness",
    "flight readiness",
    "flight-certified",
    "flight certified",
    "mission-ready",
    "mission_ready",
    "mission ready",
    "mission-readiness",
    "mission readiness",
    "mission-certified",
    "mission certified",
    "certified for flight",
    "operationally approved",
    "operationally_approved",
    "approved for operations",
    "approved_for_operations",
    "operational approval",
    "habitat-safe",
    "habitat_safe",
    "habitat safe",
    "habitat-safety approved",
    "habitat safety approved",
    "habitat-safety certified",
    "habitat safety certified",
    "medical/life-support certified",
    "medical life-support certified",
    "medical life support certified",
    "life-support-certified",
    "life_support_certified",
    "life support certified",
    "medical-use approved",
    "medical use approved",
    "medical approval",
    "regulatory-approved",
    "regulatory_approved",
    "regulatory approved",
    "regulatory approval",
    "regulated-use approved",
    "regulated_use_approved",
    "regulated use approved",
    "regulated-use approval",
    "regulated use approval",
];

const READINESS_NONCLAIM_CUES: &[&str] = &[
    " not ",
    " no ",
    " never ",
    " without ",
    " does not ",
    " doesn t ",
    " do not ",
    " must not ",
    " should not ",
    " cannot ",
    " can not ",
    " forbidden ",
    " disallowed ",
    " prohibited ",
    " block ",
    " blocks ",
    " blocked ",
    " guardrail ",
    " guardrails ",
    " avoid ",
    " avoids ",
    " avoiding ",
    " not allowed ",
    " stop if ",
    " negative statement ",
    " negative statements ",
    " caveat ",
    " caveats ",
    " non claim ",
    " non claims ",
    " nonclaim ",
    " misread as ",
    " misread ",
    " imply ",
    " implies ",
    " implied ",
    " treats ",
    " treated ",
    " allowed phrase ",
    " allowed phrases ",
    " future claim ",
    " future claims ",
];

const REQUIRED_DATA_REGISTRY_FIELDS: &[&str] = &[
    "id",
    "title",
    "local_path",
    "artifact_kind",
    "origin",
    "license",
    "hash_status",
    "allowed_use",
    "bundling_decision",
    "validation_status",
    "owner",
    "update_cadence",
    "notes",
];

fn equation_inventory_path() -> &'static str {
    "validation/equation_inventory.tsv"
}

fn equation_inventory_header() -> &'static str {
    "category\tid\tsource_path\tline\tfunction_or_ref\tstatus\tblocked\tblock_reason\trow_count"
}

fn m07_represented_function_rows() -> usize {
    1_350
}

fn external_m07_resolution_header() -> &'static str {
    "schema_version\tresolution_id\tsource_artifact_id\tclassifier_path\tsource_row_locator\tsource_row_number\trust_function_alias\tscilab_function_alias\tsource_file_locator\tformula_family\trisk_tier\trecommended_chunk_group\ttarget_formula_id\ttarget_resolution_id\ttarget_batch_manifest\ttarget_package\ttarget_crate_name\ttarget_runtime_symbol\ttarget_runtime_path\ttarget_contract_path\ttarget_validation_card_path\ttarget_source_seed_path\tvalidation_status\tdisposition\tblock_reason"
}

fn count_external_m07_processed_rows(root: &Path) -> Result<usize, String> {
    let resolution_dir = root.join("formula-vault/resolutions");
    let entries =
        fs::read_dir(&resolution_dir).map_err(|e| format!("{}: {e}", resolution_dir.display()))?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", resolution_dir.display()))?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if path.is_file() && name.starts_with("m07_") && name.ends_with(".tsv") {
            paths.push(path);
        }
    }
    paths.sort();
    if paths.is_empty() {
        return Err(format!(
            "{} has no external M07 resolution manifests",
            resolution_dir.display()
        ));
    }

    let mut source_row_locators = BTreeSet::new();
    let mut resolution_ids = BTreeSet::new();
    let mut total_rows = 0usize;
    for path in paths {
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut lines = text.lines().filter(|line| !line.trim().is_empty());
        let header = lines
            .next()
            .ok_or_else(|| format!("{} has no header", path.display()))?;
        if header != external_m07_resolution_header() {
            return Err(format!("{} has an unsupported header", path.display()));
        }
        let mut file_rows = 0usize;
        for (index, line) in lines.enumerate() {
            let line_no = index + 2;
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() != 25 {
                return Err(format!(
                    "{} line {} has {} fields; expected 25",
                    path.display(),
                    line_no,
                    fields.len()
                ));
            }
            if fields[0] != "aerocodex.external_m07_resolution.v1" {
                return Err(format!(
                    "{} line {} has unsupported schema `{}`",
                    path.display(),
                    line_no,
                    fields[0]
                ));
            }
            if fields[1].is_empty() || !resolution_ids.insert(fields[1].to_string()) {
                return Err(format!(
                    "{} line {} has an empty or duplicate resolution_id `{}`",
                    path.display(),
                    line_no,
                    fields[1]
                ));
            }
            if fields[4].is_empty() || !source_row_locators.insert(fields[4].to_string()) {
                return Err(format!(
                    "{} line {} has an empty or duplicate source_row_locator `{}`",
                    path.display(),
                    line_no,
                    fields[4]
                ));
            }
            if fields[22] != "research_required" {
                return Err(format!(
                    "{} line {} validation_status must remain research_required",
                    path.display(),
                    line_no
                ));
            }
            let disposition = fields[23];
            if disposition.is_empty()
                || disposition.contains("pending")
                || disposition.contains("unresolved")
            {
                return Err(format!(
                    "{} line {} disposition `{}` is not terminal",
                    path.display(),
                    line_no,
                    disposition
                ));
            }
            if fields[24].is_empty() {
                return Err(format!(
                    "{} line {} is missing block_reason",
                    path.display(),
                    line_no
                ));
            }
            file_rows += 1;
        }
        if file_rows == 0 {
            return Err(format!("{} has no data rows", path.display()));
        }
        total_rows = total_rows
            .checked_add(file_rows)
            .ok_or_else(|| "external M07 processed row count overflow".to_string())?;
    }
    Ok(total_rows)
}

fn governed_executable_research_equation_count() -> usize {
    152
}

fn required_status_vocabulary_names() -> &'static [&'static str] {
    &[
        "verification_status",
        "data_validation_status",
        "hash_status",
    ]
}

fn forbidden_status_vocabulary_values() -> &'static [&'static str] {
    &[
        "certified",
        "flight_ready",
        "mission_ready",
        "operationally_approved",
        "approved_for_operations",
        "habitat_safe",
        "medical_use",
        "regulated_use_approved",
    ]
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

    let result = match arg_refs.as_slice() {
        ["verify"] | ["verify", "--all"] => verify_all(),
        ["verify", "cards"] => {
            let root = repo_root();
            verify_schema(&root).and_then(|()| {
                let source_ids = collect_source_registry_ids(&root)?;
                verify_cards(&root, Some(&source_ids))
            })
        }
        ["verify", "source-registry"] => {
            let root = repo_root();
            verify_source_registry(&root).map(|_| ())
        }
        ["verify", "data-registry"] => {
            let root = repo_root();
            verify_data_registry(&root).map(|_| ())
        }
        ["verify", "status-vocabulary"] => {
            let root = repo_root();
            verify_status_vocabulary(&root)
        }
        ["verify", "formula-vault"] => {
            let root = repo_root();
            verify_formula_vault(&root)
        }
        ["verify", "equation-inventory"] => {
            let root = repo_root();
            verify_equation_inventory(&root).map(|_| ())
        }
        ["verify", "beta1"] => {
            let root = repo_root();
            verify_beta1(&root)
        }
        ["equation-batch", "plan", rest @ ..] => run_equation_batch_plan(rest),
        ["equation-batch", "report", rest @ ..] => run_equation_batch_report(rest),
        ["dependency-policy"] => dependency_policy(),
        ["help"] | ["--help"] | ["-h"] => {
            print_usage();
            Ok(())
        }
        _ => {
            print_usage();
            Err("unknown or missing command".to_string())
        }
    };

    if let Err(err) = result {
        eprintln!("xtask error: {err}");
        std::process::exit(1);
    }
}

fn run_equation_batch_plan(args: &[&str]) -> Result<(), String> {
    let root = repo_root();
    let options = equation_batch::plan::PlanOptions::parse_args(args)?;
    let report = equation_batch::plan::plan_equation_batches(&root, &options)?;
    if options.json {
        print!("{}", equation_batch::plan::render_json(&report));
    } else {
        print!("{}", equation_batch::plan::render_human(&report));
    }
    Ok(())
}

fn run_equation_batch_report(args: &[&str]) -> Result<(), String> {
    let root = repo_root();
    let options = equation_batch::report::ReportOptions::parse_args(args)?;
    equation_batch::report::run_report_command(&root, &options)
}

fn print_usage() {
    eprintln!(
        "usage:\n  cargo run -p xtask -- verify --all\n  cargo run -p xtask -- verify cards\n  cargo run -p xtask -- verify source-registry\n  cargo run -p xtask -- verify data-registry\n  cargo run -p xtask -- verify status-vocabulary\n  cargo run -p xtask -- verify formula-vault\n  cargo run -p xtask -- verify equation-inventory\n  cargo run -p xtask -- verify beta1\n  cargo run -p xtask -- equation-batch plan --manifest equation-batches/m00-canonical-units.tsv [--json]\n  cargo run -p xtask -- equation-batch plan --all-manifests [--json]\n  cargo run -p xtask -- dependency-policy"
    );
}

fn verify_equation_batch_scaffold(root: &Path) -> Result<(), String> {
    let required_files = [
        "equation-batches/README.md",
        "equation-batches/m00-canonical-units.tsv",
    ];
    for relative in required_files {
        let path = root.join(relative);
        if !path.is_file() {
            return Err(format!("missing equation-batch scaffold file: {relative}"));
        }
    }

    let batch_dir = root.join("equation-batches");
    let entries =
        fs::read_dir(&batch_dir).map_err(|error| format!("{}: {error}", batch_dir.display()))?;
    let mut manifest_count = 0usize;
    let mut row_count = 0usize;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", batch_dir.display()))?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("tsv") {
            continue;
        }
        manifest_count += 1;
        let text =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let manifest = equation_batch::manifest::parse_equation_batch_manifest(&path, &text)?;
        for row in &manifest.rows {
            if row.validation_status != "research_required" {
                return Err(format!(
                    "{} line {} validation_status must remain research_required",
                    path.display(),
                    row.line_number
                ));
            }
            for (field_name, value) in [
                ("contract_path", row.contract_path.as_str()),
                ("validation_card_path", row.validation_card_path.as_str()),
                ("source_seed_path", row.source_seed_path.as_str()),
            ] {
                if !root.join(value).is_file() {
                    return Err(format!(
                        "{} line {} {field_name} does not exist: {value}",
                        path.display(),
                        row.line_number
                    ));
                }
            }
        }
        if manifest.row_count > 40 {
            return Err(format!(
                "{} exceeds the public equation-batch row limit: {} > 40",
                path.display(),
                manifest.row_count
            ));
        }
        row_count = row_count
            .checked_add(manifest.row_count)
            .ok_or_else(|| "equation-batch row count overflow".to_string())?;
    }
    if manifest_count == 0 {
        return Err("equation-batches has no TSV manifests".to_string());
    }

    println!(
        "verified equation-batch manifests: manifests={manifest_count}; rows={row_count}; validation_status=research_required"
    );
    Ok(())
}

fn verify_beta1(root: &Path) -> Result<(), String> {
    let required_files = [
        "crates/aero-codex-cli/Cargo.toml",
        "crates/aero-codex-cli/src/main.rs",
        "crates/aero-codex-cli/tests/cli.rs",
        "docs/beta1/release_concept.md",
        "docs/beta1/cli_quickstart.md",
        "docs/beta1/release_testing.md",
    ];
    for relative in required_files {
        let path = root.join(relative);
        if !path.is_file() {
            return Err(format!("missing Beta 1 artifact: {}", path.display()));
        }
    }

    let workspace_manifest = fs::read_to_string(root.join("Cargo.toml"))
        .map_err(|error| format!("Cargo.toml: {error}"))?;
    for marker in [
        "\"crates/aero-codex-cli\"",
        "version = \"0.0.1\"",
        "rust-version = \"1.74\"",
    ] {
        if !workspace_manifest.contains(marker) {
            return Err(format!("Cargo.toml missing Beta 1 marker `{marker}`"));
        }
    }

    let cli_manifest = fs::read_to_string(root.join("crates/aero-codex-cli/Cargo.toml"))
        .map_err(|error| format!("aero-codex-cli Cargo.toml: {error}"))?;
    for marker in [
        "name = \"aero-codex-cli\"",
        "name = \"aerocodex\"",
        "aero-codex-astrodynamics",
        "aero-codex-core",
    ] {
        if !cli_manifest.contains(marker) {
            return Err(format!(
                "crates/aero-codex-cli/Cargo.toml missing marker `{marker}`"
            ));
        }
    }

    let cli_source = fs::read_to_string(root.join("crates/aero-codex-cli/src/main.rs"))
        .map_err(|error| format!("aero-codex-cli main.rs: {error}"))?;
    for marker in [
        "fn release_channel() -> &'static str",
        "env!(\"CARGO_PKG_VERSION\")",
        "fn build_commit() -> &'static str",
        "fn build_target() -> &'static str",
        "fn build_profile() -> &'static str",
        "fn validation_status() -> &'static str",
        "fn supported_formula_count() -> usize",
        "formula_vault.m00.canonical.time_unit_from_mu_du",
        "formula_vault.m00.canonical.speed_from_canonical",
        "self-check",
        "fn safety_notice() -> &'static str",
    ] {
        if !cli_source.contains(marker) {
            return Err(format!(
                "crates/aero-codex-cli/src/main.rs missing marker `{marker}`"
            ));
        }
    }

    let release_concept = fs::read_to_string(root.join("docs/beta1/release_concept.md"))
        .map_err(|error| format!("Beta 1 release concept: {error}"))?;
    for marker in [
        "Beta 1 concept",
        "Cargo version remains `0.0.1`",
        "research_required",
        "not certified",
        "ten canonical-unit formulas",
        "1,000+",
    ] {
        if !release_concept.contains(marker) {
            return Err(format!(
                "docs/beta1/release_concept.md missing marker `{marker}`"
            ));
        }
    }

    let release_testing = fs::read_to_string(root.join("docs/beta1/release_testing.md"))
        .map_err(|error| format!("Beta 1 release testing: {error}"))?;
    for marker in [
        "Public Rust-only release-candidate check",
        "workspace-local and path-only",
        "not an aerospace assurance or certification gate",
    ] {
        if !release_testing.contains(marker) {
            return Err(format!(
                "docs/beta1/release_testing.md missing marker `{marker}`"
            ));
        }
    }

    let bash_friend_test = fs::read_to_string(root.join("scripts/friend_test_local.sh"))
        .map_err(|error| format!("Bash friend test: {error}"))?;
    let powershell_friend_test = fs::read_to_string(root.join("scripts/friend_test_local.ps1"))
        .map_err(|error| format!("PowerShell friend test: {error}"))?;
    let smoke_marker = "cargo run -p aero-codex-cli -- self-check --json";
    for (name, text) in [
        ("scripts/friend_test_local.sh", bash_friend_test),
        ("scripts/friend_test_local.ps1", powershell_friend_test),
    ] {
        if !text.contains(smoke_marker) {
            return Err(format!("{name} missing Beta 1 self-check command"));
        }
    }

    println!(
        "verified Beta 1 concept: channel=beta1-concept; cargo_version=0.0.1; supported_formulas=10; validation_status=research_required; release_packaging=not_public_repo_tracked"
    );
    Ok(())
}

fn verify_public_wording_guardrails(root: &Path) -> Result<(), String> {
    let guardrails_path = root.join(PUBLIC_WORDING_GUARDRAILS_PATH);
    if !guardrails_path.is_file() {
        return Err(format!(
            "missing public wording guardrails: {}",
            guardrails_path.display()
        ));
    }

    let guardrails = fs::read_to_string(&guardrails_path)
        .map_err(|error| format!("{}: {error}", guardrails_path.display()))?;
    if !guardrails.contains(APPROVED_PUBLIC_WORDING) {
        return Err(format!(
            "{} missing approved public wording",
            guardrails_path.display()
        ));
    }
    for phrase in REQUIRED_PUBLIC_FORBIDDEN_CLAIM_PHRASES {
        if !guardrails.to_ascii_lowercase().contains(phrase) {
            return Err(format!(
                "{} missing forbidden public-claim phrase `{phrase}`",
                guardrails_path.display()
            ));
        }
    }

    let readme_path = root.join("README.md");
    let readme = fs::read_to_string(&readme_path)
        .map_err(|error| format!("{}: {error}", readme_path.display()))?;
    if !readme.contains(APPROVED_PUBLIC_WORDING) {
        return Err(format!(
            "{} missing approved public wording",
            readme_path.display()
        ));
    }

    let mut scanned_files = 0usize;
    for relative in PUBLIC_WORDING_SCAN_PATHS {
        let path = root.join(relative);
        if path.is_file() {
            if is_public_wording_scan_file(&path) {
                scan_public_wording_file(&path)?;
                scanned_files += 1;
            }
        } else if path.is_dir() {
            visit_files(&path, &mut |candidate| {
                if is_public_wording_scan_file(candidate) {
                    scan_public_wording_file(candidate)?;
                    scanned_files += 1;
                }
                Ok(())
            })?;
        } else {
            return Err(format!(
                "missing public wording scan path: {}",
                path.display()
            ));
        }
    }

    println!(
        "verified public wording guardrails: approved_wording=present; scanned_files={scanned_files}"
    );
    Ok(())
}

fn is_public_wording_scan_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension == "md" || extension == "txt")
}

fn scan_public_wording_file(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    verify_no_forbidden_readiness_markers(path, &text)
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf()
}

fn verify_all() -> Result<(), String> {
    let root = repo_root();
    verify_schema(&root)?;
    let source_ids = verify_source_registry(&root)?;
    verify_cards(&root, Some(&source_ids))?;
    verify_data_registry(&root)?;
    verify_status_vocabulary(&root)?;
    verify_public_wording_guardrails(&root)?;
    verify_formula_vault(&root)?;
    verify_equation_inventory(&root)?;
    verify_equation_batch_scaffold(&root)?;
    verify_beta1(&root)?;
    Ok(())
}

fn verify_schema(root: &Path) -> Result<(), String> {
    let schema = root.join("validation/schema/codex_card.schema.json");
    if !schema.is_file() {
        return Err(format!("missing schema: {}", schema.display()));
    }

    let text = fs::read_to_string(&schema).map_err(|e| format!("{}: {e}", schema.display()))?;
    for marker in REQUIRED_SCHEMA_MARKERS {
        if !text.contains(marker) {
            return Err(format!(
                "{} missing schema marker `{marker}`",
                schema.display()
            ));
        }
    }
    for status in ALLOWED_STATUSES {
        if !text.contains(status) {
            return Err(format!("{} missing status `{status}`", schema.display()));
        }
    }
    for category in ALLOWED_CARD_CATEGORIES {
        if !text.contains(category) {
            return Err(format!(
                "{} missing card category `{category}`",
                schema.display()
            ));
        }
    }

    println!("verified Codex Card schema scaffold");
    Ok(())
}

fn verify_cards(root: &Path, source_ids: Option<&BTreeSet<String>>) -> Result<(), String> {
    let cards_dir = root.join("validation/cards");
    if !cards_dir.is_dir() {
        return Err(format!(
            "missing validation cards directory: {}",
            cards_dir.display()
        ));
    }

    let mut count = 0usize;
    visit_yaml(&cards_dir, &mut |path| {
        count += 1;
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        verify_no_forbidden_readiness_markers(path, &text)?;
        verify_required_top_level_fields(path, &text, REQUIRED_CARD_FIELDS)?;

        let id = require_top_level_value(path, &text, "id")?;
        if !is_valid_dotted_id(id) {
            return Err(format!("{} has invalid dotted id `{id}`", path.display()));
        }

        let name = require_top_level_value(path, &text, "name")?;
        if name.is_empty() {
            return Err(format!("{} has empty name", path.display()));
        }

        let category = require_top_level_value(path, &text, "category")?;
        require_allowed(path, "category", category, ALLOWED_CARD_CATEGORIES)?;

        let status = require_top_level_value(path, &text, "status")?;
        require_allowed(path, "status", status, ALLOWED_STATUSES)?;

        for field in REQUIRED_CARD_LIST_FIELDS {
            require_nonempty_list(path, &text, field)?;
        }

        let source_id = require_nested_value(path, &text, "source", "id")?;
        if !source_id.starts_with("source.") || !is_valid_dotted_id(source_id) {
            return Err(format!(
                "{} has invalid source id `{source_id}`",
                path.display()
            ));
        }
        if let Some(known_sources) = source_ids {
            if !known_sources.contains(source_id) {
                return Err(format!(
                    "{} references source id `{source_id}` without a matching source-registry seed",
                    path.display()
                ));
            }
        }

        let source_status = require_nested_value(path, &text, "source", "status")?;
        require_allowed(path, "source.status", source_status, ALLOWED_STATUSES)?;

        let notes = require_top_level_value(path, &text, "notes")?;
        if notes.is_empty() {
            return Err(format!("{} has empty notes", path.display()));
        }

        Ok(())
    })?;

    if count == 0 {
        return Err("no validation cards found".to_string());
    }

    println!("verified {count} validation cards");
    Ok(())
}

fn verify_source_registry(root: &Path) -> Result<BTreeSet<String>, String> {
    let registry_dir = root.join("validation/source_registry");
    if !registry_dir.is_dir() {
        return Err(format!(
            "missing source registry directory: {}",
            registry_dir.display()
        ));
    }

    let mut count = 0usize;
    let mut ids = BTreeSet::new();
    visit_yaml(&registry_dir, &mut |path| {
        count += 1;
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        verify_no_forbidden_readiness_markers(path, &text)?;
        verify_required_top_level_fields(path, &text, REQUIRED_SOURCE_REGISTRY_FIELDS)?;

        let id = require_top_level_value(path, &text, "id")?;
        if !id.starts_with("source.") || !is_valid_dotted_id(id) {
            return Err(format!(
                "{} has invalid source-registry id `{id}`",
                path.display()
            ));
        }
        if !ids.insert(id.to_string()) {
            return Err(format!("duplicate source-registry id `{id}`"));
        }

        let title = require_top_level_value(path, &text, "title")?;
        if title.is_empty() {
            return Err(format!("{} has empty title", path.display()));
        }

        let category = require_top_level_value(path, &text, "category")?;
        require_allowed(path, "category", category, ALLOWED_SOURCE_CATEGORIES)?;

        let status = require_top_level_value(path, &text, "status")?;
        require_allowed(path, "status", status, ALLOWED_STATUSES)?;

        for field in REQUIRED_SOURCE_REGISTRY_LIST_FIELDS {
            require_nonempty_list(path, &text, field)?;
        }

        Ok(())
    })?;

    if count == 0 {
        return Err("no source-registry YAML files found".to_string());
    }

    println!("verified {count} source-registry seeds");
    Ok(ids)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DataRegistryEntry {
    line: usize,
    fields: BTreeMap<String, String>,
}

impl DataRegistryEntry {
    fn get(&self, field: &str) -> Option<&str> {
        self.fields.get(field).map(String::as_str)
    }
}

fn verify_data_registry(root: &Path) -> Result<Vec<DataRegistryEntry>, String> {
    let registry = root.join("data-governance/DATA_REGISTRY.yaml");
    if !registry.is_file() {
        return Err(format!("missing data registry: {}", registry.display()));
    }

    let text = fs::read_to_string(&registry).map_err(|e| format!("{}: {e}", registry.display()))?;
    let entries = verify_data_registry_text(&registry, &text)?;
    for entry in &entries {
        let local_path = entry.get("local_path").unwrap_or_default();
        if is_external_path(local_path) {
            continue;
        }
        let candidate = root.join(local_path);
        if !candidate.exists() {
            return Err(format!(
                "{} entry `{}` local_path `{local_path}` does not exist in the repository",
                registry.display(),
                entry.get("id").unwrap_or("<missing>")
            ));
        }
    }

    println!(
        "verified {} data-governance registry entries",
        entries.len()
    );
    Ok(entries)
}

fn verify_data_registry_text(path: &Path, text: &str) -> Result<Vec<DataRegistryEntry>, String> {
    let entries = parse_data_registry_entries(path, text)?;
    let mut ids = BTreeSet::new();

    for entry in &entries {
        for field in REQUIRED_DATA_REGISTRY_FIELDS {
            match entry.get(field) {
                Some(value) if !value.is_empty() => {}
                _ => {
                    return Err(format!(
                        "{} entry starting line {} missing required field `{field}:`",
                        path.display(),
                        entry.line
                    ));
                }
            }
        }

        let id = entry.get("id").unwrap_or_default();
        if !is_valid_artifact_id(id) {
            return Err(format!(
                "{} entry starting line {} has invalid artifact id `{id}`",
                path.display(),
                entry.line
            ));
        }
        if !ids.insert(id.to_string()) {
            return Err(format!("duplicate data-registry artifact id `{id}`"));
        }

        let local_path = entry.get("local_path").unwrap_or_default();
        validate_data_registry_local_path(path, entry, local_path)?;

        let hash_status = entry.get("hash_status").unwrap_or_default();
        match entry.get("sha256") {
            Some(hash) if !hash.is_empty() => {
                if !is_valid_sha256(hash) {
                    return Err(format!(
                        "{} entry `{id}` has invalid sha256 `{hash}`",
                        path.display()
                    ));
                }
            }
            _ => {
                if !is_pending_hash_status(hash_status) {
                    return Err(format!(
                        "{} entry `{id}` missing `sha256:` without `hash_status: pending_with_reason`",
                        path.display()
                    ));
                }
            }
        }

        if is_external_archive(entry) && has_unsafe_external_archive_decision(entry) {
            return Err(format!(
                "{} entry `{id}` has unsafe external archive import decision",
                path.display()
            ));
        }
    }

    Ok(entries)
}

fn parse_data_registry_entries(path: &Path, text: &str) -> Result<Vec<DataRegistryEntry>, String> {
    let mut entries = Vec::new();
    let mut current: Option<DataRegistryEntry> = None;
    let mut saw_artifacts = false;

    for (index, raw_line) in text.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed == "artifacts:" {
            if saw_artifacts {
                return Err(format!(
                    "{} line {line_no}: duplicate `artifacts:` root",
                    path.display()
                ));
            }
            saw_artifacts = true;
            continue;
        }
        if !saw_artifacts {
            return Err(format!(
                "{} line {line_no}: expected top-level `artifacts:` before registry entries",
                path.display()
            ));
        }

        if let Some(rest) = trimmed.strip_prefix("- ") {
            finish_data_registry_entry(path, &mut entries, current.take())?;
            let mut entry = DataRegistryEntry {
                line: line_no,
                fields: BTreeMap::new(),
            };
            if !rest.trim().is_empty() {
                insert_data_registry_field(path, &mut entry, line_no, rest.trim())?;
            }
            current = Some(entry);
            continue;
        }

        if trimmed == "-" {
            finish_data_registry_entry(path, &mut entries, current.take())?;
            current = Some(DataRegistryEntry {
                line: line_no,
                fields: BTreeMap::new(),
            });
            continue;
        }

        if raw_line.starts_with(' ') || raw_line.starts_with('\t') {
            let entry = current.as_mut().ok_or_else(|| {
                format!(
                    "{} line {line_no}: registry field appears before an artifact entry",
                    path.display()
                )
            })?;
            insert_data_registry_field(path, entry, line_no, trimmed)?;
            continue;
        }

        return Err(format!(
            "{} line {line_no}: malformed data-registry line `{trimmed}`",
            path.display()
        ));
    }

    finish_data_registry_entry(path, &mut entries, current.take())?;
    if entries.is_empty() {
        return Err(format!("{} has no data-registry entries", path.display()));
    }

    Ok(entries)
}

fn finish_data_registry_entry(
    path: &Path,
    entries: &mut Vec<DataRegistryEntry>,
    entry: Option<DataRegistryEntry>,
) -> Result<(), String> {
    if let Some(entry) = entry {
        if entry.fields.is_empty() {
            return Err(format!(
                "{} entry starting line {} is empty or malformed",
                path.display(),
                entry.line
            ));
        }
        entries.push(entry);
    }
    Ok(())
}

fn insert_data_registry_field(
    path: &Path,
    entry: &mut DataRegistryEntry,
    line_no: usize,
    text: &str,
) -> Result<(), String> {
    let (field, raw_value) = text.split_once(':').ok_or_else(|| {
        format!(
            "{} line {line_no}: malformed registry field `{text}`",
            path.display()
        )
    })?;
    let field = field.trim();
    if field.is_empty()
        || !field
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(format!(
            "{} line {line_no}: invalid registry field name `{field}`",
            path.display()
        ));
    }
    let value = clean_scalar(raw_value).to_string();
    if entry.fields.insert(field.to_string(), value).is_some() {
        return Err(format!(
            "{} line {line_no}: duplicate field `{field}:` in one registry entry",
            path.display()
        ));
    }
    Ok(())
}

fn validate_data_registry_local_path(
    path: &Path,
    entry: &DataRegistryEntry,
    local_path: &str,
) -> Result<(), String> {
    if is_external_path(local_path) {
        return Ok(());
    }
    let lowered = local_path.to_ascii_lowercase();
    if local_path.starts_with('/')
        || lowered.starts_with("c:\\")
        || lowered.contains("c:\\users")
        || lowered.starts_with("/mnt/")
        || local_path.split('/').any(|part| part == "..")
        || local_path.split('\\').any(|part| part == "..")
    {
        return Err(format!(
            "{} entry `{}` local_path `{local_path}` must be repo-relative or external://stage4/...",
            path.display(),
            entry.get("id").unwrap_or("<missing>")
        ));
    }
    Ok(())
}

fn is_external_path(value: &str) -> bool {
    value.starts_with("external://")
}

fn is_pending_hash_status(value: &str) -> bool {
    value
        .to_ascii_lowercase()
        .starts_with("pending_with_reason")
}

fn is_valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn is_valid_artifact_id(value: &str) -> bool {
    if value.is_empty()
        || value.starts_with(['.', '_', '-'])
        || value.ends_with(['.', '_', '-'])
        || value.contains("..")
    {
        return false;
    }
    value.chars().all(|ch| {
        ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '_' || ch == '-'
    })
}

fn is_external_archive(entry: &DataRegistryEntry) -> bool {
    let local_path = entry.get("local_path").unwrap_or_default();
    let artifact_kind = entry
        .get("artifact_kind")
        .unwrap_or_default()
        .to_ascii_lowercase();
    is_external_path(local_path)
        && (artifact_kind.contains("archive") || local_path.to_ascii_lowercase().ends_with(".zip"))
}

fn has_unsafe_external_archive_decision(entry: &DataRegistryEntry) -> bool {
    let allowed_use = entry
        .get("allowed_use")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let bundling_decision = entry
        .get("bundling_decision")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let blocked_markers = [
        "direct public api import",
        "public api import",
        "import archive into public crates",
        "import into public crates",
        "merge into public crates",
        "bundled in public crates",
        "bulk import into crates",
    ];
    [allowed_use.as_str(), bundling_decision.as_str()]
        .iter()
        .any(|value| blocked_markers.iter().any(|marker| value.contains(marker)))
}

fn verify_status_vocabulary(root: &Path) -> Result<(), String> {
    let vocabulary_path = root.join("validation/status_vocabulary.yaml");
    if !vocabulary_path.is_file() {
        return Err(format!(
            "missing status vocabulary: {}",
            vocabulary_path.display()
        ));
    }
    let vocabulary_text = fs::read_to_string(&vocabulary_path)
        .map_err(|e| format!("{}: {e}", vocabulary_path.display()))?;

    let mut card_texts: Vec<(String, String)> = Vec::new();
    visit_yaml(&root.join("validation/cards"), &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        card_texts.push((relative_display(root, path), text));
        Ok(())
    })?;
    let card_refs: Vec<(&str, &str)> = card_texts
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();

    let mut source_texts: Vec<(String, String)> = Vec::new();
    visit_yaml(&root.join("validation/source_registry"), &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        source_texts.push((relative_display(root, path), text));
        Ok(())
    })?;
    let source_refs: Vec<(&str, &str)> = source_texts
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();

    let data_registry_path = root.join("data-governance/DATA_REGISTRY.yaml");
    let data_registry_text = fs::read_to_string(&data_registry_path)
        .map_err(|e| format!("{}: {e}", data_registry_path.display()))?;

    verify_status_vocabulary_text(
        &vocabulary_path,
        &vocabulary_text,
        &card_refs,
        &source_refs,
        &data_registry_path,
        &data_registry_text,
    )?;

    println!(
        "verified status vocabulary against {} validation cards, {} source-registry seeds, and data-governance registry",
        card_refs.len(),
        source_refs.len()
    );
    Ok(())
}

fn verify_status_vocabulary_text(
    vocabulary_path: &Path,
    vocabulary_text: &str,
    card_texts: &[(&str, &str)],
    source_texts: &[(&str, &str)],
    data_registry_path: &Path,
    data_registry_text: &str,
) -> Result<(), String> {
    let vocabulary = parse_status_vocabulary(vocabulary_path, vocabulary_text)?;
    for name in required_status_vocabulary_names() {
        let values = vocabulary.get(*name).ok_or_else(|| {
            format!(
                "{} missing required status vocabulary `{name}`",
                vocabulary_path.display()
            )
        })?;
        if values.is_empty() {
            return Err(format!(
                "{} status vocabulary `{name}` has no allowed values",
                vocabulary_path.display()
            ));
        }
        for value in values {
            if forbidden_status_vocabulary_values().contains(&value.as_str()) {
                return Err(format!(
                    "{} contains forbidden readiness status `{value}`",
                    vocabulary_path.display()
                ));
            }
        }
    }

    let verification_status = vocabulary.get("verification_status").unwrap();
    for status in ALLOWED_STATUSES {
        if !verification_status.contains(*status) {
            return Err(format!(
                "{} missing verification status `{status}`",
                vocabulary_path.display()
            ));
        }
    }
    for status in verification_status {
        if !ALLOWED_STATUSES.contains(&status.as_str()) {
            return Err(format!(
                "{} has unsupported verification status vocabulary value `{status}`",
                vocabulary_path.display()
            ));
        }
    }

    let data_validation_status = vocabulary.get("data_validation_status").unwrap();
    let hash_status = vocabulary.get("hash_status").unwrap();

    for (name, text) in card_texts {
        let path = Path::new(name);
        let status = require_top_level_value(path, text, "status")?;
        require_status_value(name, "status", status, verification_status)?;
        let source_status = require_nested_value(path, text, "source", "status")?;
        require_status_value(name, "source.status", source_status, verification_status)?;
    }

    for (name, text) in source_texts {
        let path = Path::new(name);
        let status = require_top_level_value(path, text, "status")?;
        require_status_value(name, "status", status, verification_status)?;
    }

    let entries = verify_data_registry_text(data_registry_path, data_registry_text)?;
    for entry in entries {
        let id = entry.get("id").unwrap_or("<missing>");
        let validation_status = entry.get("validation_status").unwrap_or_default();
        if !data_validation_status.contains(validation_status) {
            return Err(format!(
                "data-registry entry `{id}` field `validation_status` has unsupported status `{validation_status}`"
            ));
        }
        let entry_hash_status = entry.get("hash_status").unwrap_or_default();
        if !hash_status.contains(entry_hash_status) && !is_pending_hash_status(entry_hash_status) {
            return Err(format!(
                "data-registry entry `{id}` field `hash_status` has unsupported status `{entry_hash_status}`"
            ));
        }
    }

    Ok(())
}

fn parse_status_vocabulary(
    path: &Path,
    text: &str,
) -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let mut vocabulary: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut current_vocabulary: Option<String> = None;
    let mut in_allowed_values = false;

    for (index, raw_line) in text.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if is_top_level_line(raw_line) {
            let Some(name) = trimmed.strip_suffix(':') else {
                return Err(format!(
                    "{} line {line_no}: malformed status vocabulary root `{trimmed}`",
                    path.display()
                ));
            };
            if name.is_empty()
                || !name
                    .chars()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
            {
                return Err(format!(
                    "{} line {line_no}: invalid status vocabulary name `{name}`",
                    path.display()
                ));
            }
            current_vocabulary = Some(name.to_string());
            in_allowed_values = false;
            vocabulary.entry(name.to_string()).or_default();
            continue;
        }

        if trimmed == "allowed_values:" {
            if current_vocabulary.is_none() {
                return Err(format!(
                    "{} line {line_no}: `allowed_values:` appears before a vocabulary root",
                    path.display()
                ));
            }
            in_allowed_values = true;
            continue;
        }

        if let Some(raw_value) = trimmed.strip_prefix("- ") {
            let name = current_vocabulary.as_ref().ok_or_else(|| {
                format!(
                    "{} line {line_no}: status value appears before a vocabulary root",
                    path.display()
                )
            })?;
            if !in_allowed_values {
                return Err(format!(
                    "{} line {line_no}: status value appears outside `allowed_values:`",
                    path.display()
                ));
            }
            let value = clean_scalar(raw_value);
            if value.is_empty() {
                return Err(format!(
                    "{} line {line_no}: empty status vocabulary value",
                    path.display()
                ));
            }
            vocabulary
                .get_mut(name)
                .expect("current vocabulary should exist")
                .insert(value.to_string());
            continue;
        }

        return Err(format!(
            "{} line {line_no}: malformed status vocabulary line `{trimmed}`",
            path.display()
        ));
    }

    if vocabulary.is_empty() {
        return Err(format!("{} has no status vocabularies", path.display()));
    }
    Ok(vocabulary)
}

fn require_status_value(
    path: &str,
    field: &str,
    value: &str,
    allowed: &BTreeSet<String>,
) -> Result<(), String> {
    if allowed.contains(value) {
        Ok(())
    } else {
        Err(format!(
            "{path} field `{field}` has unsupported status `{value}`"
        ))
    }
}

fn relative_display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FormulaVaultCandidateSummary {
    slice_id: String,
    formula_ids: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PublicFunctionRef {
    source_path: String,
    line: usize,
    function: String,
}

impl PublicFunctionRef {
    fn new(source_path: String, line: usize, function: String) -> Self {
        Self {
            source_path,
            line,
            function,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EquationInventoryExpectedCounts {
    executable_research_equations: usize,
    metadata_only_candidates: usize,
    external_m07_processed_rows: usize,
    external_m07_backlog_rows: usize,
    validation_cards: usize,
    source_registry_seeds: usize,
    validation_card_only_records: usize,
    helper_algorithms: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct EquationInventorySummary {
    executable_research_equations: usize,
    metadata_only_candidates: usize,
    external_m07_processed_rows: usize,
    external_m07_backlog_rows: usize,
    validation_cards: usize,
    source_registry_seeds: usize,
    validation_card_only_records: usize,
    helper_algorithms: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EquationInventoryRow {
    line_no: usize,
    category: String,
    id: String,
    source_path: String,
    line: usize,
    function_or_ref: String,
    status: String,
    blocked: String,
    block_reason: String,
    row_count: usize,
}

fn verify_equation_inventory(root: &Path) -> Result<EquationInventorySummary, String> {
    let inventory_path = root.join(equation_inventory_path());
    if !inventory_path.is_file() {
        return Err(format!(
            "missing equation inventory: {}",
            inventory_path.display()
        ));
    }

    let text = fs::read_to_string(&inventory_path)
        .map_err(|e| format!("{}: {e}", inventory_path.display()))?;
    let public_functions = collect_public_functions(root)?;
    let metadata_only_candidates = collect_formula_vault_formula_ids(root)?.len();
    let validation_cards = count_yaml_files(&root.join("validation/cards"))?;
    let source_registry_seeds = count_yaml_files(&root.join("validation/source_registry"))?;
    let governed_executable_count = governed_executable_research_equation_count();
    let helper_algorithms = public_functions
        .len()
        .checked_sub(governed_executable_count)
        .ok_or_else(|| {
            format!(
                "public function count {} is below governed executable equation count {}",
                public_functions.len(),
                governed_executable_count
            )
        })?;
    let external_m07_processed_rows = count_external_m07_processed_rows(root)?;
    let selected_external_rows = metadata_only_candidates
        .checked_add(external_m07_processed_rows)
        .ok_or_else(|| "selected external row count overflow".to_string())?;
    let expected = EquationInventoryExpectedCounts {
        executable_research_equations: governed_executable_count,
        metadata_only_candidates,
        external_m07_processed_rows,
        external_m07_backlog_rows: m07_represented_function_rows()
            .checked_sub(selected_external_rows)
            .ok_or_else(|| "selected external rows exceed m07 represented rows".to_string())?,
        validation_cards,
        source_registry_seeds,
        validation_card_only_records: validation_cards,
        helper_algorithms,
    };

    let summary =
        verify_equation_inventory_text(&inventory_path, &text, &expected, Some(&public_functions))?;
    println!(
        "verified equation inventory: executable_research_equations={}; metadata_only_candidates={}; external_m07_processed_rows={}; external_m07_backlog_rows={}; validation_cards={}; source_registry_seeds={}; validation_card_only_records={}; helper_algorithms={}",
        summary.executable_research_equations,
        summary.metadata_only_candidates,
        summary.external_m07_processed_rows,
        summary.external_m07_backlog_rows,
        summary.validation_cards,
        summary.source_registry_seeds,
        summary.validation_card_only_records,
        summary.helper_algorithms
    );
    Ok(summary)
}

fn verify_equation_inventory_text(
    path: &Path,
    text: &str,
    expected: &EquationInventoryExpectedCounts,
    public_functions: Option<&BTreeSet<PublicFunctionRef>>,
) -> Result<EquationInventorySummary, String> {
    verify_no_forbidden_readiness_markers(path, text)?;
    let rows = parse_equation_inventory_rows(path, text)?;
    if rows.is_empty() {
        return Err(format!("{} has no inventory rows", path.display()));
    }

    let mut ids = BTreeSet::new();
    let mut public_rows = BTreeSet::new();
    let mut summary = EquationInventorySummary {
        validation_cards: expected.validation_cards,
        source_registry_seeds: expected.source_registry_seeds,
        ..EquationInventorySummary::default()
    };

    for row in &rows {
        if !ids.insert(row.id.clone()) {
            return Err(format!(
                "{} line {} duplicate equation inventory id `{}`",
                path.display(),
                row.line_no,
                row.id
            ));
        }
        if !is_valid_artifact_id(&row.id) {
            return Err(format!(
                "{} line {} invalid equation inventory id `{}`",
                path.display(),
                row.line_no,
                row.id
            ));
        }
        require_allowed(path, "status", &row.status, ALLOWED_STATUSES)?;
        if row.blocked != "true" {
            return Err(format!(
                "{} line {} inventory item `{}` must remain blocked",
                path.display(),
                row.line_no,
                row.id
            ));
        }
        if row.block_reason.is_empty() {
            return Err(format!(
                "{} line {} inventory item `{}` missing block_reason",
                path.display(),
                row.line_no,
                row.id
            ));
        }
        let external_backlog_zero_closure =
            row.category == "external_m07_backlog_row" && expected.external_m07_backlog_rows == 0;
        if row.row_count == 0 && !external_backlog_zero_closure {
            return Err(format!(
                "{} line {} inventory item `{}` row_count must be positive",
                path.display(),
                row.line_no,
                row.id
            ));
        }

        match row.category.as_str() {
            "executable_research_equation" => {
                require_unit_row_count(path, row)?;
                require_repo_source_path(path, row, "crates/")?;
                require_positive_line(path, row)?;
                summary.executable_research_equations += row.row_count;
                public_rows.insert(PublicFunctionRef::new(
                    row.source_path.clone(),
                    row.line,
                    row.function_or_ref.clone(),
                ));
            }
            "metadata_only_formula_vault_candidate" => {
                require_unit_row_count(path, row)?;
                require_repo_source_path(path, row, "formula-vault/candidates/")?;
                if !row.function_or_ref.starts_with("formula_vault.") {
                    return Err(format!(
                        "{} line {} metadata candidate `{}` must reference a formula_vault id",
                        path.display(),
                        row.line_no,
                        row.id
                    ));
                }
                summary.metadata_only_candidates += row.row_count;
            }
            "external_m07_processed_row" => {
                require_repo_source_path(path, row, "formula-vault/resolutions/")?;
                if row.line != 0 {
                    return Err(format!(
                        "{} line {} external processed row `{}` must use line 0",
                        path.display(),
                        row.line_no,
                        row.id
                    ));
                }
                summary.external_m07_processed_rows += row.row_count;
            }
            "external_m07_backlog_row" => {
                if !row.source_path.starts_with("external://stage4/") {
                    return Err(format!(
                        "{} line {} external backlog `{}` must use an external://stage4 path",
                        path.display(),
                        row.line_no,
                        row.id
                    ));
                }
                summary.external_m07_backlog_rows += row.row_count;
            }
            "validation_card_only_record" => {
                require_unit_row_count(path, row)?;
                require_repo_source_path(path, row, "validation/cards/")?;
                summary.validation_card_only_records += row.row_count;
            }
            "helper_algorithm" => {
                require_unit_row_count(path, row)?;
                require_repo_source_path(path, row, "crates/")?;
                require_positive_line(path, row)?;
                summary.helper_algorithms += row.row_count;
                public_rows.insert(PublicFunctionRef::new(
                    row.source_path.clone(),
                    row.line,
                    row.function_or_ref.clone(),
                ));
            }
            other => {
                return Err(format!(
                    "{} line {} unsupported equation inventory category `{other}`",
                    path.display(),
                    row.line_no
                ));
            }
        }
    }

    if summary.executable_research_equations != expected.executable_research_equations {
        return Err(format!(
            "{} executable_research_equation count {} does not match expected {}",
            path.display(),
            summary.executable_research_equations,
            expected.executable_research_equations
        ));
    }
    if summary.metadata_only_candidates != expected.metadata_only_candidates {
        return Err(format!(
            "{} metadata_only_formula_vault_candidate count {} does not match expected {}",
            path.display(),
            summary.metadata_only_candidates,
            expected.metadata_only_candidates
        ));
    }
    if summary.external_m07_processed_rows != expected.external_m07_processed_rows {
        return Err(format!(
            "{} external_m07_processed_row count {} does not match expected {}",
            path.display(),
            summary.external_m07_processed_rows,
            expected.external_m07_processed_rows
        ));
    }
    if summary.external_m07_backlog_rows != expected.external_m07_backlog_rows {
        return Err(format!(
            "{} external_m07_backlog_row count {} does not match expected {}",
            path.display(),
            summary.external_m07_backlog_rows,
            expected.external_m07_backlog_rows
        ));
    }
    if summary.validation_card_only_records != expected.validation_card_only_records {
        return Err(format!(
            "{} validation_card_only_record count {} does not match expected {}",
            path.display(),
            summary.validation_card_only_records,
            expected.validation_card_only_records
        ));
    }
    if summary.helper_algorithms != expected.helper_algorithms {
        return Err(format!(
            "{} helper_algorithm count {} does not match expected {}",
            path.display(),
            summary.helper_algorithms,
            expected.helper_algorithms
        ));
    }

    if let Some(expected_public_functions) = public_functions {
        for function_ref in expected_public_functions {
            if !public_rows.contains(function_ref) {
                return Err(format!(
                    "{} missing public function inventory row for {}:{}:{}",
                    path.display(),
                    function_ref.source_path,
                    function_ref.line,
                    function_ref.function
                ));
            }
        }
        for function_ref in &public_rows {
            if !expected_public_functions.contains(function_ref) {
                return Err(format!(
                    "{} has inventory row for unknown public function {}:{}:{}",
                    path.display(),
                    function_ref.source_path,
                    function_ref.line,
                    function_ref.function
                ));
            }
        }
    }

    Ok(summary)
}

fn parse_equation_inventory_rows(
    path: &Path,
    text: &str,
) -> Result<Vec<EquationInventoryRow>, String> {
    let mut lines = text.lines().filter(|line| !line.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| format!("{} missing equation inventory header", path.display()))?;
    if header != equation_inventory_header() {
        return Err(format!(
            "{} has unsupported equation inventory header `{header}`",
            path.display()
        ));
    }

    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate().skip(1) {
        let line_no = index + 1;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 9 {
            return Err(format!(
                "{} line {line_no} expected 9 tab-separated fields, found {}",
                path.display(),
                fields.len()
            ));
        }
        let line_field = fields[3].parse::<usize>().map_err(|e| {
            format!(
                "{} line {line_no} has invalid source line `{}`: {e}",
                path.display(),
                fields[3]
            )
        })?;
        let row_count = fields[8].parse::<usize>().map_err(|e| {
            format!(
                "{} line {line_no} has invalid row_count `{}`: {e}",
                path.display(),
                fields[8]
            )
        })?;
        rows.push(EquationInventoryRow {
            line_no,
            category: fields[0].trim().to_string(),
            id: fields[1].trim().to_string(),
            source_path: fields[2].trim().to_string(),
            line: line_field,
            function_or_ref: fields[4].trim().to_string(),
            status: fields[5].trim().to_string(),
            blocked: fields[6].trim().to_string(),
            block_reason: fields[7].trim().to_string(),
            row_count,
        });
    }
    Ok(rows)
}

fn require_unit_row_count(path: &Path, row: &EquationInventoryRow) -> Result<(), String> {
    if row.row_count == 1 {
        Ok(())
    } else {
        Err(format!(
            "{} line {} inventory item `{}` must use row_count 1 for category `{}`",
            path.display(),
            row.line_no,
            row.id,
            row.category
        ))
    }
}

fn require_repo_source_path(
    path: &Path,
    row: &EquationInventoryRow,
    prefix: &str,
) -> Result<(), String> {
    if row.source_path.starts_with(prefix) && !row.source_path.contains("..") {
        Ok(())
    } else {
        Err(format!(
            "{} line {} inventory item `{}` source_path `{}` must start with `{prefix}`",
            path.display(),
            row.line_no,
            row.id,
            row.source_path
        ))
    }
}

fn require_positive_line(path: &Path, row: &EquationInventoryRow) -> Result<(), String> {
    if row.line > 0 {
        Ok(())
    } else {
        Err(format!(
            "{} line {} inventory item `{}` must record a positive source line",
            path.display(),
            row.line_no,
            row.id
        ))
    }
}

fn count_yaml_files(dir: &Path) -> Result<usize, String> {
    let mut count = 0usize;
    visit_yaml(dir, &mut |_| {
        count += 1;
        Ok(())
    })?;
    Ok(count)
}

fn collect_formula_vault_formula_ids(root: &Path) -> Result<BTreeSet<String>, String> {
    let source_ids = collect_source_registry_ids(root)?;
    let card_ids = collect_validation_card_ids(root)?;
    let candidates_dir = root.join("formula-vault/candidates");
    let mut candidate_texts: Vec<(String, String)> = Vec::new();
    visit_yaml(&candidates_dir, &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        candidate_texts.push((relative_display(root, path), text));
        Ok(())
    })?;
    let candidate_refs: Vec<(&str, &str)> = candidate_texts
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let summaries = verify_formula_vault_candidate_texts(&candidate_refs, &source_ids, &card_ids)?;
    let mut formula_ids = BTreeSet::new();
    for summary in summaries {
        for formula_id in summary.formula_ids {
            formula_ids.insert(formula_id);
        }
    }
    Ok(formula_ids)
}

fn collect_public_functions(root: &Path) -> Result<BTreeSet<PublicFunctionRef>, String> {
    let crates_dir = root.join("crates");
    let mut functions = BTreeSet::new();
    visit_files(&crates_dir, &mut |path| {
        if !path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext == "rs")
        {
            return Ok(());
        }
        if !path
            .components()
            .any(|component| component.as_os_str() == "src")
        {
            return Ok(());
        }
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let rel = relative_display(root, path);
        for (index, line) in text.lines().enumerate() {
            if let Some(function) = parse_pub_fn_name(line) {
                functions.insert(PublicFunctionRef::new(rel.clone(), index + 1, function));
            }
        }
        Ok(())
    })?;
    Ok(functions)
}

fn parse_pub_fn_name(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let rest = trimmed
        .strip_prefix("pub fn ")
        .or_else(|| trimmed.strip_prefix("pub const fn "))?;
    let name: String = rest
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn verify_formula_vault(root: &Path) -> Result<(), String> {
    let vault_dir = root.join("formula-vault");
    if !vault_dir.is_dir() {
        return Err(format!(
            "missing formula-vault metadata directory: {}",
            vault_dir.display()
        ));
    }

    let candidates_dir = vault_dir.join("candidates");
    if !candidates_dir.is_dir() {
        return Err(format!(
            "missing formula-vault candidates directory: {}",
            candidates_dir.display()
        ));
    }

    let source_ids = collect_source_registry_ids(root)?;
    let card_ids = collect_validation_card_ids(root)?;
    let mut candidate_texts: Vec<(String, String)> = Vec::new();
    visit_yaml(&candidates_dir, &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        candidate_texts.push((relative_display(root, path), text));
        Ok(())
    })?;

    if candidate_texts.is_empty() {
        return Err("no formula-vault candidate metadata records found".to_string());
    }

    let candidate_refs: Vec<(&str, &str)> = candidate_texts
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let summaries = verify_formula_vault_candidate_texts(&candidate_refs, &source_ids, &card_ids)?;
    println!(
        "verified {} formula-vault candidate metadata records",
        summaries.len()
    );
    Ok(())
}

fn verify_formula_vault_candidate_texts(
    candidates: &[(&str, &str)],
    source_ids: &BTreeSet<String>,
    card_ids: &BTreeSet<String>,
) -> Result<Vec<FormulaVaultCandidateSummary>, String> {
    let mut summaries = Vec::new();
    let mut slice_ids = BTreeSet::new();
    let mut formula_ids = BTreeSet::new();

    for (name, text) in candidates {
        let path = Path::new(name);
        let summary = verify_formula_vault_candidate_text(path, text, source_ids, card_ids)?;
        if !slice_ids.insert(summary.slice_id.clone()) {
            return Err(format!(
                "duplicate formula-vault slice id `{}`",
                summary.slice_id
            ));
        }
        for formula_id in &summary.formula_ids {
            if !formula_ids.insert(formula_id.clone()) {
                return Err(format!(
                    "duplicate formula-vault formula id `{formula_id}` across candidate records"
                ));
            }
        }
        summaries.push(summary);
    }

    Ok(summaries)
}

fn verify_formula_vault_candidate_text(
    path: &Path,
    text: &str,
    source_ids: &BTreeSet<String>,
    card_ids: &BTreeSet<String>,
) -> Result<FormulaVaultCandidateSummary, String> {
    verify_no_forbidden_readiness_markers(path, text)?;
    verify_formula_vault_no_local_evidence_paths(path, text)?;
    verify_required_top_level_fields(path, text, required_formula_vault_top_level_fields())?;

    let schema_version = require_top_level_value(path, text, "schema_version")?;
    if schema_version != "formula_vault_candidate_slice.v1" {
        return Err(format!(
            "{} has unsupported formula-vault schema_version `{schema_version}`",
            path.display()
        ));
    }

    let record_status = require_top_level_value(path, text, "record_status")?;
    if record_status == "template_only" || !record_status.contains("research_required") {
        return Err(format!(
            "{} record_status `{record_status}` must remain a research_required candidate record",
            path.display()
        ));
    }

    for field in required_formula_vault_slice_fields() {
        require_nested_value(path, text, "slice", field)?;
    }
    let slice_id = require_nested_value(path, text, "slice", "id")?;
    if !slice_id.starts_with("formula_vault.") || !is_valid_dotted_id(slice_id) {
        return Err(format!(
            "{} has invalid formula-vault slice id `{slice_id}`",
            path.display()
        ));
    }
    let public_surface = require_nested_value(path, text, "slice", "public_api_surface")?;
    if !public_surface.contains("blocked") {
        return Err(format!(
            "{} slice.public_api_surface must remain blocked",
            path.display()
        ));
    }

    for field in required_formula_vault_sources_fields() {
        require_nested_value(path, text, "sources", field)?;
    }
    require_nested_list(path, text, "sources", "source_artifact_ids")?;

    let source_id = require_nested_value(path, text, "sources", "source_registry_seed_id")?;
    if !source_ids.contains(source_id) {
        return Err(format!(
            "{} references source-registry seed `{source_id}` without a matching validation/source_registry file",
            path.display()
        ));
    }
    let card_id = require_nested_value(path, text, "sources", "validation_card_id")?;
    if !card_ids.contains(card_id) {
        return Err(format!(
            "{} references validation card `{card_id}` without a matching validation/cards file",
            path.display()
        ));
    }

    for field in required_formula_vault_contract_list_fields() {
        require_nested_list(path, text, "formula_contract", field)?;
    }
    let formula_ids = require_nested_list(path, text, "formula_contract", "formula_ids")?;
    let mut unique_formula_ids = BTreeSet::new();
    for formula_id in formula_ids {
        if !formula_id.starts_with("formula_vault.") || !is_valid_dotted_id(&formula_id) {
            return Err(format!(
                "{} has invalid formula-vault formula id `{formula_id}`",
                path.display()
            ));
        }
        if !unique_formula_ids.insert(formula_id.clone()) {
            return Err(format!(
                "{} duplicate formula-vault formula id `{formula_id}`",
                path.display()
            ));
        }
    }

    for field in required_formula_vault_validation_fields() {
        require_nested_value(path, text, "validation_records", field)?;
    }
    let required_source = require_nested_value(
        path,
        text,
        "validation_records",
        "required_source_registry_seed",
    )?;
    if required_source != source_id {
        return Err(format!(
            "{} validation_records.required_source_registry_seed `{required_source}` does not match sources.source_registry_seed_id `{source_id}`",
            path.display()
        ));
    }
    let required_card =
        require_nested_value(path, text, "validation_records", "required_validation_card")?;
    if required_card != card_id {
        return Err(format!(
            "{} validation_records.required_validation_card `{required_card}` does not match sources.validation_card_id `{card_id}`",
            path.display()
        ));
    }
    let status = require_nested_value(path, text, "validation_records", "status")?;
    require_allowed(path, "validation_records.status", status, ALLOWED_STATUSES)?;

    require_nested_value(path, text, "evidence_plan", "tolerance_rationale")?;
    let default_state = require_nested_value(path, text, "promotion_gate", "default_state")?;
    if default_state != "blocked" {
        return Err(format!(
            "{} promotion_gate.default_state must remain blocked",
            path.display()
        ));
    }
    require_nested_list(path, text, "promotion_gate", "required_before_promotion")?;

    for field in required_formula_vault_non_claims_true() {
        let value = require_nested_value(path, text, "non_claims", field)?;
        if value != "true" {
            return Err(format!(
                "{} non_claims.{field} must be true",
                path.display()
            ));
        }
    }

    Ok(FormulaVaultCandidateSummary {
        slice_id: slice_id.to_string(),
        formula_ids: unique_formula_ids,
    })
}

fn verify_formula_vault_no_local_evidence_paths(path: &Path, text: &str) -> Result<(), String> {
    let lowered = text.to_ascii_lowercase();
    let forbidden = ["/mnt/", "c:\\users", "evidence/logs", "target/", "target\\"];
    for marker in forbidden {
        if lowered.contains(marker) {
            return Err(format!(
                "{} contains forbidden formula-vault local/evidence path marker `{marker}`",
                path.display()
            ));
        }
    }
    Ok(())
}

fn require_nested_list(
    path: &Path,
    text: &str,
    section: &str,
    field: &str,
) -> Result<Vec<String>, String> {
    let items = nested_list_items(text, section, field);
    if items.is_empty() {
        Err(format!(
            "{} nested list `{section}.{field}:` must contain at least one item",
            path.display()
        ))
    } else {
        Ok(items)
    }
}

fn nested_list_items(text: &str, section: &str, field: &str) -> Vec<String> {
    let section_prefix = format!("{section}:");
    let field_prefix = format!("{field}:");
    let mut in_section = false;
    let mut in_field = false;
    let mut items = Vec::new();

    for line in text.lines() {
        let trimmed_end = line.trim_end();
        if is_top_level_line(trimmed_end) {
            in_section = trimmed_end.starts_with(&section_prefix);
            in_field = false;
            continue;
        }
        if !in_section {
            continue;
        }

        let nested = trimmed_end.trim_start();
        let indent = trimmed_end.len().saturating_sub(nested.len());
        if indent <= 2 && nested.starts_with(&field_prefix) {
            in_field = true;
            continue;
        }
        if indent <= 2 && !nested.starts_with("- ") && nested.contains(':') {
            in_field = false;
            continue;
        }
        if in_field {
            if let Some(item) = nested.strip_prefix("- ") {
                let item = clean_scalar(item);
                if !item.is_empty() {
                    items.push(item.to_string());
                }
            }
        }
    }

    items
}

fn dependency_policy() -> Result<(), String> {
    let root = repo_root();
    let mut tomls = Vec::new();
    visit_files(&root, &mut |path| {
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "Cargo.toml")
        {
            tomls.push(path.to_path_buf());
        }
        Ok(())
    })?;

    for toml in tomls {
        let text = fs::read_to_string(&toml).map_err(|e| format!("{}: {e}", toml.display()))?;
        let lowered = text.to_ascii_lowercase();
        for token in FORBIDDEN_DEPENDENCY_TOKENS {
            if lowered.contains(token) {
                return Err(format!(
                    "{} contains forbidden token `{token}`",
                    toml.display()
                ));
            }
        }
        for line in lowered.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name") || trimmed.starts_with('[') {
                continue;
            }
            if trimmed.contains("-sys") {
                return Err(format!(
                    "{} appears to reference a -sys crate",
                    toml.display()
                ));
            }
        }
    }
    println!("dependency policy check passed");
    Ok(())
}

fn verify_required_top_level_fields(
    path: &Path,
    text: &str,
    fields: &[&str],
) -> Result<(), String> {
    for field in fields {
        if !has_top_level_field(text, field) {
            return Err(format!(
                "{} missing required field `{field}:`",
                path.display()
            ));
        }
    }
    Ok(())
}

fn verify_no_forbidden_readiness_markers(path: &Path, text: &str) -> Result<(), String> {
    if let Some((line_number, marker)) = forbidden_readiness_claim_marker(text) {
        return Err(format!(
            "{} line {} contains forbidden readiness marker `{marker}`",
            path.display(),
            line_number
        ));
    }
    Ok(())
}

fn forbidden_readiness_claim_marker(text: &str) -> Option<(usize, &'static str)> {
    let mut nonclaim_context_indent: Option<usize> = None;
    let mut nonclaim_markdown_section = false;
    let mut wrapped_nonclaim_context_remaining = 0usize;
    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        let indent = line.len().saturating_sub(line.trim_start().len());
        let in_wrapped_nonclaim_context = wrapped_nonclaim_context_remaining > 0;
        wrapped_nonclaim_context_remaining = wrapped_nonclaim_context_remaining.saturating_sub(1);
        if trimmed.starts_with('#') {
            nonclaim_markdown_section = is_nonclaim_context_heading(trimmed);
            nonclaim_context_indent = None;
        } else if !trimmed.is_empty()
            && nonclaim_context_indent.is_some_and(|context_indent| indent <= context_indent)
        {
            nonclaim_context_indent = None;
        }
        if is_nonclaim_context_heading(trimmed) {
            if trimmed.starts_with('#') {
                nonclaim_markdown_section = true;
            } else {
                nonclaim_context_indent = Some(indent);
            }
        }

        let normalized_line = normalize_readiness_text(line);
        for marker in FORBIDDEN_READINESS_MARKERS {
            let normalized_marker = normalize_readiness_text(marker);
            let mut search_start = 0usize;
            while let Some(relative_index) =
                normalized_line[search_start..].find(&normalized_marker)
            {
                let marker_index = search_start + relative_index;
                let in_nonclaim_context = nonclaim_markdown_section
                    || nonclaim_context_indent.is_some()
                    || in_wrapped_nonclaim_context;
                if is_normalized_phrase_boundary(
                    &normalized_line,
                    marker_index,
                    normalized_marker.len(),
                ) && !in_nonclaim_context
                    && !is_allowed_readiness_nonclaim_context(&normalized_line, marker_index)
                {
                    return Some((line_index + 1, marker));
                }
                search_start = marker_index + 1;
            }
        }
        if starts_wrapped_readiness_nonclaim_context(&normalized_line, line) {
            wrapped_nonclaim_context_remaining = 1;
        }
    }
    None
}

fn is_nonclaim_context_heading(trimmed_line: &str) -> bool {
    let normalized_heading = normalize_readiness_text(trimmed_line.trim_end_matches(':'));
    matches!(
        normalized_heading.as_str(),
        "allowed non claim phrases"
            | "allowed phrases"
            | "blocked use now"
            | "excluded cases"
            | "exclusions"
            | "explicit non scope"
            | "forbidden positive claim markers"
            | "limits"
            | "limitations"
            | "non claim phrases"
            | "non claims"
            | "non scope"
    )
}

fn is_normalized_phrase_boundary(line: &str, start: usize, length: usize) -> bool {
    let bytes = line.as_bytes();
    let end = start + length;
    let before_ok = start == 0 || bytes[start - 1] == b' ';
    let after_ok = end == bytes.len() || bytes[end] == b' ';
    before_ok && after_ok
}

fn normalize_readiness_text(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut previous_was_space = true;
    for character in text.chars() {
        let mapped = if character.is_ascii_alphanumeric() {
            character.to_ascii_lowercase()
        } else {
            ' '
        };
        if mapped == ' ' {
            if !previous_was_space {
                normalized.push(' ');
                previous_was_space = true;
            }
        } else {
            normalized.push(mapped);
            previous_was_space = false;
        }
    }
    normalized.trim().to_string()
}

fn is_allowed_readiness_nonclaim_context(normalized_line: &str, marker_index: usize) -> bool {
    let prefix = &normalized_line[..marker_index];
    let suffix = &normalized_line[marker_index..];
    let padded_prefix = format!(" {prefix} ");
    let padded_suffix = format!(" {suffix} ");
    if padded_prefix.contains(" keep ")
        && (padded_suffix.contains(" caveat ") || padded_suffix.contains(" caveats "))
    {
        return true;
    }
    normalized_text_has_nonclaim_cue(prefix)
}

fn normalized_text_has_nonclaim_cue(normalized_text: &str) -> bool {
    let padded_text = format!(" {normalized_text} ");
    READINESS_NONCLAIM_CUES
        .iter()
        .any(|cue| padded_text.contains(cue))
}

fn starts_wrapped_readiness_nonclaim_context(normalized_line: &str, raw_line: &str) -> bool {
    let allows_wrapped_phrase = raw_line.trim_end().ends_with(',')
        || [
            " flight",
            " mission",
            " operational",
            " regulated",
            " regulated use",
        ]
        .iter()
        .any(|suffix| normalized_line.ends_with(suffix));
    if !allows_wrapped_phrase {
        return false;
    }
    let padded_line = format!(" {normalized_line} ");
    let fixed_cues = [
        " does not establish ",
        " does not imply ",
        " does not provide ",
        " does not currently provide ",
        " not certified ",
        " no certification ",
        " no external source parity claim ",
    ];
    if fixed_cues.iter().any(|cue| padded_line.contains(cue)) {
        return true;
    }
    padded_line.contains(" adds no ")
        && [" parity ", " certification ", " claim ", " approval "]
            .iter()
            .any(|cue| padded_line.contains(cue))
}

fn require_top_level_value<'a>(path: &Path, text: &'a str, field: &str) -> Result<&'a str, String> {
    top_level_value(text, field)
        .ok_or_else(|| format!("{} missing top-level value `{field}:`", path.display()))
}

fn require_nested_value<'a>(
    path: &Path,
    text: &'a str,
    section: &str,
    field: &str,
) -> Result<&'a str, String> {
    nested_value(text, section, field).ok_or_else(|| {
        format!(
            "{} missing nested value `{section}.{field}`",
            path.display()
        )
    })
}

fn require_nonempty_list(path: &Path, text: &str, field: &str) -> Result<(), String> {
    if list_has_item(text, field) {
        Ok(())
    } else {
        Err(format!(
            "{} field `{field}:` must contain at least one list item",
            path.display()
        ))
    }
}

fn require_allowed(path: &Path, field: &str, value: &str, allowed: &[&str]) -> Result<(), String> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "{} field `{field}` has unsupported value `{value}`",
            path.display()
        ))
    }
}

fn is_top_level_line(line: &str) -> bool {
    !line.is_empty() && !line.starts_with(' ') && !line.starts_with('\t')
}

fn has_top_level_field(text: &str, field: &str) -> bool {
    let prefix = format!("{field}:");
    text.lines().any(|line| {
        let trimmed = line.trim_end();
        is_top_level_line(trimmed) && trimmed.starts_with(&prefix)
    })
}

fn top_level_value<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    let prefix = format!("{field}:");
    text.lines().find_map(|line| {
        let trimmed = line.trim_end();
        if is_top_level_line(trimmed) && trimmed.starts_with(&prefix) {
            Some(clean_scalar(&trimmed[prefix.len()..]))
        } else {
            None
        }
    })
}

fn nested_value<'a>(text: &'a str, section: &str, field: &str) -> Option<&'a str> {
    let section_prefix = format!("{section}:");
    let field_prefix = format!("{field}:");
    let mut in_section = false;

    for line in text.lines() {
        let trimmed_end = line.trim_end();
        let top_level = is_top_level_line(trimmed_end);
        if top_level {
            in_section = trimmed_end.starts_with(&section_prefix);
            continue;
        }
        if in_section {
            let nested = trimmed_end.trim_start();
            if nested.starts_with(&field_prefix) {
                return Some(clean_scalar(&nested[field_prefix.len()..]));
            }
        }
    }

    None
}

fn list_has_item(text: &str, field: &str) -> bool {
    let prefix = format!("{field}:");
    let mut in_list = false;

    for line in text.lines() {
        let trimmed_end = line.trim_end();
        let top_level = is_top_level_line(trimmed_end);
        if top_level {
            in_list = trimmed_end.starts_with(&prefix);
            continue;
        }
        if in_list {
            let nested = trimmed_end.trim_start();
            if let Some(item) = nested.strip_prefix("- ") {
                if !item.trim().is_empty() {
                    return true;
                }
            }
        }
    }

    false
}

fn clean_scalar(raw: &str) -> &str {
    raw.trim().trim_matches('"').trim_matches('\'').trim()
}

fn is_valid_dotted_id(value: &str) -> bool {
    if value.is_empty() || value.starts_with('.') || value.ends_with('.') || value.contains("..") {
        return false;
    }

    value.split('.').all(|segment| {
        !segment.is_empty()
            && segment
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    })
}

fn collect_source_registry_ids(root: &Path) -> Result<BTreeSet<String>, String> {
    let registry_dir = root.join("validation/source_registry");
    let mut ids = BTreeSet::new();
    visit_yaml(&registry_dir, &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let id = require_top_level_value(path, &text, "id")?;
        ids.insert(id.to_string());
        Ok(())
    })?;
    Ok(ids)
}

fn collect_validation_card_ids(root: &Path) -> Result<BTreeSet<String>, String> {
    let cards_dir = root.join("validation/cards");
    let mut ids = BTreeSet::new();
    visit_yaml(&cards_dir, &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let id = require_top_level_value(path, &text, "id")?;
        ids.insert(id.to_string());
        Ok(())
    })?;
    Ok(ids)
}

fn visit_yaml<F>(dir: &Path, f: &mut F) -> Result<(), String>
where
    F: FnMut(&Path) -> Result<(), String>,
{
    visit_files(dir, &mut |path| {
        if path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            f(path)?;
        }
        Ok(())
    })
}

fn visit_files<F>(dir: &Path, f: &mut F) -> Result<(), String>
where
    F: FnMut(&Path) -> Result<(), String>,
{
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if name == "target" || name == ".git" {
            continue;
        }
        if path.is_dir() {
            visit_files(&path, f)?;
        } else {
            f(&path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_level_value_ignores_nested_values() {
        let text = "status: research_required\nsource:\n  status: implementation_verified\n";
        assert_eq!(top_level_value(text, "status"), Some("research_required"));
        assert_eq!(
            nested_value(text, "source", "status"),
            Some("implementation_verified")
        );
    }

    #[test]
    fn list_detection_requires_nonempty_item() {
        let missing = "assumptions:\n  -   \noutputs:\n  - value\n";
        let present = "assumptions:\n  - finite scalar inputs\n";
        assert!(!list_has_item(missing, "assumptions"));
        assert!(list_has_item(present, "assumptions"));
    }

    #[test]
    fn dotted_id_validation_rejects_unsafe_shapes() {
        assert!(is_valid_dotted_id(
            "life_support.bioregenerative.closure_fraction"
        ));
        assert!(is_valid_dotted_id(
            "source.gasdynamics.naca_report_1135.research_required"
        ));
        assert!(!is_valid_dotted_id("LifeSupport.closure"));
        assert!(!is_valid_dotted_id("life_support..closure"));
        assert!(!is_valid_dotted_id("life-support.closure"));
    }

    #[test]
    fn forbidden_readiness_claim_examples_fail() {
        for claim in [
            "AeroCodex is NASA-ready for public use.",
            "status: flight_ready\n",
            "This module is operationally approved.",
            "The package is medical/life-support certified.",
            "Engineering note says this is a release,\nflight readiness achieved.",
        ] {
            assert!(
                forbidden_readiness_claim_marker(claim).is_some(),
                "forbidden readiness claim should fail: {claim}"
            );
        }
    }

    #[test]
    fn forbidden_readiness_approved_nonclaim_passes() {
        for disclaimer in [
            APPROVED_PUBLIC_WORDING,
            "AeroCodex is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.",
            "Forbidden marker: NASA-ready must not be used as a public readiness claim.",
            "Any future claim of certified, flight-ready, mission-ready, or operationally suitable behavior must be backed by a separate assurance process.",
            "## Blocked use now\n- claiming habitat safety, medical suitability, operational readiness, certification, or regulated-use approval;",
            "# This linkage does not establish Scilab parity, certification,\n# flight readiness, mission readiness, operational approval, or regulated-use approval.",
            "# This linkage adds no external parity, certification,\n# flight-readiness, mission-readiness, operational, or regulated-use claim.",
            "# This linkage adds no Scilab parity, certification, flight\n# readiness, mission readiness, operational approval, or regulated-use approval.",
        ] {
            assert_eq!(
                forbidden_readiness_claim_marker(disclaimer),
                None,
                "approved non-claim wording should pass: {disclaimer}"
            );
        }
    }

    fn minimal_data_registry_entry(id: &str) -> String {
        format!(
            "artifacts:\n  - id: {id}\n    title: Minimal registry fixture\n    local_path: data/example.txt\n    artifact_kind: repo_file\n    origin: unit_test\n    license: MIT OR Apache-2.0\n    sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n    hash_status: sha256_verified\n    allowed_use: documentation and source-governance fixture only\n    bundling_decision: bundled repo-relative governance fixture\n    validation_status: registered_fixture_only\n    owner: AeroCodex maintainers\n    update_cadence: per governance update\n    notes: Minimal valid dependency-free parser fixture.\n"
        )
    }

    fn assert_data_registry_error_contains(text: &str, expected: &str) {
        let err = verify_data_registry_text(Path::new("DATA_REGISTRY.yaml"), text)
            .expect_err("registry fixture should fail");
        assert!(
            err.contains(expected),
            "expected error containing `{expected}`, got `{err}`"
        );
    }

    #[test]
    fn valid_minimal_data_registry_entry_passes() {
        let text = minimal_data_registry_entry("artifact.valid_minimal");
        let entries = verify_data_registry_text(Path::new("DATA_REGISTRY.yaml"), &text)
            .expect("minimal registry entry should pass");
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn duplicate_data_registry_id_fails() {
        let text = format!(
            "{}{}",
            minimal_data_registry_entry("artifact.duplicate"),
            minimal_data_registry_entry("artifact.duplicate").replace("artifacts:\n", "")
        );
        assert_data_registry_error_contains(&text, "duplicate data-registry artifact id");
    }

    #[test]
    fn missing_data_registry_local_path_fails() {
        let text = minimal_data_registry_entry("artifact.missing_path")
            .replace("    local_path: data/example.txt\n", "");
        assert_data_registry_error_contains(&text, "missing required field `local_path:`");
    }

    #[test]
    fn missing_data_registry_hash_without_pending_reason_fails() {
        let text = minimal_data_registry_entry("artifact.missing_hash").replace(
            "    sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n",
            "",
        );
        assert_data_registry_error_contains(
            &text,
            "missing `sha256:` without `hash_status: pending_with_reason`",
        );
    }

    #[test]
    fn missing_data_registry_license_or_status_fails() {
        let missing_license = minimal_data_registry_entry("artifact.missing_license")
            .replace("    license: MIT OR Apache-2.0\n", "");
        assert_data_registry_error_contains(&missing_license, "missing required field `license:`");

        let missing_status = minimal_data_registry_entry("artifact.missing_status")
            .replace("    validation_status: registered_fixture_only\n", "");
        assert_data_registry_error_contains(
            &missing_status,
            "missing required field `validation_status:`",
        );
    }

    #[test]
    fn external_archive_with_public_import_decision_fails() {
        let text = minimal_data_registry_entry("stage4.unsafe_external_archive")
            .replace(
                "    local_path: data/example.txt\n",
                "    local_path: external://stage4/unsafe.zip\n",
            )
            .replace(
                "    artifact_kind: repo_file\n",
                "    artifact_kind: external_archive\n",
            )
            .replace(
                "    allowed_use: documentation and source-governance fixture only\n",
                "    allowed_use: direct public API import into AeroCodex crates\n",
            )
            .replace(
                "    bundling_decision: bundled repo-relative governance fixture\n",
                "    bundling_decision: import archive into public crates\n",
            );
        assert_data_registry_error_contains(&text, "unsafe external archive import decision");
    }

    fn minimal_formula_vault_candidate(slice_id: &str) -> String {
        format!(
            "schema_version: formula_vault_candidate_slice.v1\nrecord_status: metadata_only_research_required\nslice:\n  id: {slice_id}\n  title: Minimal formula-vault fixture\n  lifecycle_label: equation_contract_drafted\n  stage4_chunk: stage4_chunk7c_fixture\n  public_api_surface: blocked_no_public_application_programming_interface\n  implementation_surface: blocked_no_executable_formula_code\nsources:\n  source_artifact_ids:\n    - stage4.m07_rust_port_v14.2026_06_15\n  source_registry_seed_id: source.formula_vault.fixture.research_required\n  validation_card_id: validation.formula_vault.fixture\n  human_review_locators:\n    function_rows: []\n  source_boundary:\n    m07_archive_status: external_quarantine_reference_only\n    import_policy: no_bulk_import_no_generated_binary_no_raw_source_promotion\nformula_contract:\n  formula_ids:\n    - formula_vault.m00.angle.fixture\n  variables:\n    - angle\n  units:\n    - radian\n  coordinate_frames:\n    - not_frame_dependent\n  time_scales:\n    - not_time_scale_dependent\n  sign_conventions:\n    - positive convention pending source review\n  valid_domain:\n    - finite real scalar inputs only\n  singularities:\n    - none identified for metadata fixture\n  invalid_regions:\n    - non-finite inputs excluded\n  branch_behavior:\n    - single branch fixture\n  tolerance_policy:\n    absolute_tolerance: pending_future_equivalence_plan\n    relative_tolerance: pending_future_equivalence_plan\n    rationale: deferred until future evidence\nvalidation_records:\n  required_source_registry_seed: source.formula_vault.fixture.research_required\n  required_validation_card: validation.formula_vault.fixture\n  status: research_required\n  status_upgrade_policy: blocked_without_source_review_equivalence_and_reference_evidence\nevidence_plan:\n  rust_gates:\n    - future implementation-authorized tests only\n  scilab_equivalence_jobs: []\n  sgp4_reference_checks: []\n  analytical_identity_checks: []\n  reference_oracle_checks: []\n  fixture_hashes: []\n  tolerance_rationale: deferred_until_future_equivalence_evidence\npromotion_gate:\n  default_state: blocked\n  required_before_promotion:\n    - source_registry_seed_exists\n    - validation_card_exists\n    - no_raw_m07_source_or_generated_binary_promoted\n    - no_public_application_programming_interface_promoted_without_explicit_slice_authorization\nnon_claims:\n  no_formula_implementation: true\n  no_source_translation: true\n  no_scilab_execution_result: true\n  no_certification_evidence: true\n  no_flight_readiness: true\n  no_mission_readiness: true\n  no_operational_approval: true\n  no_regulated_use_approval: true\n  no_bulk_m07_import: true\n  no_external_parity_claim_without_evidence: true\n"
        )
    }

    fn formula_vault_fixture_source_ids() -> BTreeSet<String> {
        BTreeSet::from(["source.formula_vault.fixture.research_required".to_string()])
    }

    fn formula_vault_fixture_card_ids() -> BTreeSet<String> {
        BTreeSet::from(["validation.formula_vault.fixture".to_string()])
    }

    fn assert_formula_vault_candidate_error_contains(text: &str, expected: &str) {
        let err = verify_formula_vault_candidate_text(
            Path::new("formula-vault/candidates/fixture.yaml"),
            text,
            &formula_vault_fixture_source_ids(),
            &formula_vault_fixture_card_ids(),
        )
        .expect_err("formula-vault candidate fixture should fail");
        assert!(
            err.contains(expected),
            "expected error containing `{expected}`, got `{err}`"
        );
    }

    #[test]
    fn valid_formula_vault_candidate_fixture_passes() {
        let text = minimal_formula_vault_candidate("formula_vault.m00.angle.fixture_slice");
        let summary = verify_formula_vault_candidate_text(
            Path::new("formula-vault/candidates/fixture.yaml"),
            &text,
            &formula_vault_fixture_source_ids(),
            &formula_vault_fixture_card_ids(),
        )
        .expect("minimal formula-vault candidate should pass");
        assert_eq!(summary.slice_id, "formula_vault.m00.angle.fixture_slice");
        assert_eq!(summary.formula_ids.len(), 1);
    }

    #[test]
    fn missing_formula_vault_candidate_field_fails() {
        let text = minimal_formula_vault_candidate("formula_vault.m00.angle.missing_title")
            .replace("  title: Minimal formula-vault fixture\n", "");
        assert_formula_vault_candidate_error_contains(&text, "missing nested value `slice.title`");
    }

    #[test]
    fn duplicate_formula_vault_formula_id_fails() {
        let text = minimal_formula_vault_candidate("formula_vault.m00.angle.duplicate_formula")
            .replace(
                "  formula_ids:\n    - formula_vault.m00.angle.fixture\n",
                "  formula_ids:\n    - formula_vault.m00.angle.fixture\n    - formula_vault.m00.angle.fixture\n",
            );
        assert_formula_vault_candidate_error_contains(&text, "duplicate formula-vault formula id");
    }

    #[test]
    fn duplicate_formula_vault_slice_id_fails() {
        let first = minimal_formula_vault_candidate("formula_vault.m00.angle.duplicate_slice");
        let second = minimal_formula_vault_candidate("formula_vault.m00.angle.duplicate_slice")
            .replace(
                "formula_vault.m00.angle.fixture",
                "formula_vault.m00.angle.other_fixture",
            );
        let err = verify_formula_vault_candidate_texts(
            &[
                ("formula-vault/candidates/first.yaml", first.as_str()),
                ("formula-vault/candidates/second.yaml", second.as_str()),
            ],
            &formula_vault_fixture_source_ids(),
            &formula_vault_fixture_card_ids(),
        )
        .expect_err("duplicate slice ids should fail");
        assert!(err.contains("duplicate formula-vault slice id"));
    }

    fn minimal_equation_inventory_fixture() -> String {
        "category\tid\tsource_path\tline\tfunction_or_ref\tstatus\tblocked\tblock_reason\trow_count\n\
executable_research_equation\texecutable.fixture.dynamic_pressure\tcrates/aero-codex-aerodynamics/src/lib.rs\t130\tdynamic_pressure\tresearch_required\ttrue\tresearch_preliminary_only_no_certification_or_operational_evidence\t1\n\
metadata_only_formula_vault_candidate\tformula_vault.m00.angle.fixture\tformula-vault/candidates/fixture.yaml\t0\tformula_vault.m00.angle.fixture\tresearch_required\ttrue\tmetadata_only_no_formula_implementation_no_source_import_no_public_interface\t1\n\
external_m07_processed_row\tstage4.m07.processed.fixture\tformula-vault/resolutions/fixture.tsv\t0\tfixture_terminal_disposition\tresearch_required\ttrue\tprocessed_external_row_fixture\t1\n\
external_m07_backlog_row\tstage4.m07.remaining_backlog.fixture\texternal://stage4/aerocodex_rust_port_v14_m07_final_bundle.zip\t0\tremaining_m07_rows_after_metadata_candidates_and_processed_rows\tresearch_required\ttrue\texternal_quarantine_m07_rows_not_imported_minus_selected_metadata_candidates_and_terminal_dispositions\t2\n\
validation_card_only_record\tvalidation.fixture.card\tvalidation/cards/fixture.yaml\t0\tvalidation.fixture.card\tresearch_required\ttrue\tvalidation_metadata_only_not_formula_implementation\t1\n\
helper_algorithm\thelper.fixture.ensure_finite\tcrates/aero-codex-core/src/validation.rs\t2\tensure_finite\tresearch_required\ttrue\tsupport_algorithm_not_counted_as_executable_research_equation\t1\n".to_string()
    }

    fn equation_inventory_expected_counts() -> EquationInventoryExpectedCounts {
        EquationInventoryExpectedCounts {
            executable_research_equations: 1,
            metadata_only_candidates: 1,
            external_m07_processed_rows: 1,
            external_m07_backlog_rows: 2,
            validation_cards: 1,
            source_registry_seeds: 1,
            validation_card_only_records: 1,
            helper_algorithms: 1,
        }
    }

    #[test]
    fn valid_equation_inventory_fixture_passes() {
        let public_functions = BTreeSet::from([
            PublicFunctionRef::new(
                "crates/aero-codex-aerodynamics/src/lib.rs".to_string(),
                130,
                "dynamic_pressure".to_string(),
            ),
            PublicFunctionRef::new(
                "crates/aero-codex-core/src/validation.rs".to_string(),
                2,
                "ensure_finite".to_string(),
            ),
        ]);
        let summary = verify_equation_inventory_text(
            Path::new("validation/equation_inventory.tsv"),
            &minimal_equation_inventory_fixture(),
            &equation_inventory_expected_counts(),
            Some(&public_functions),
        )
        .expect("minimal equation inventory should pass");
        assert_eq!(summary.executable_research_equations, 1);
        assert_eq!(summary.external_m07_processed_rows, 1);
        assert_eq!(summary.external_m07_backlog_rows, 2);
    }

    #[test]
    fn equation_inventory_missing_block_reason_fails() {
        let text = minimal_equation_inventory_fixture().replace(
            "research_preliminary_only_no_certification_or_operational_evidence",
            "",
        );
        let err = verify_equation_inventory_text(
            Path::new("validation/equation_inventory.tsv"),
            &text,
            &equation_inventory_expected_counts(),
            None,
        )
        .expect_err("missing block reason should fail");
        assert!(err.contains("missing block_reason"));
    }

    #[test]
    fn equation_inventory_public_function_coverage_fails() {
        let public_functions = BTreeSet::from([
            PublicFunctionRef::new(
                "crates/aero-codex-aerodynamics/src/lib.rs".to_string(),
                130,
                "dynamic_pressure".to_string(),
            ),
            PublicFunctionRef::new(
                "crates/aero-codex-core/src/validation.rs".to_string(),
                2,
                "ensure_finite".to_string(),
            ),
            PublicFunctionRef::new(
                "crates/aero-codex-core/src/validation.rs".to_string(),
                14,
                "ensure_positive".to_string(),
            ),
        ]);
        let err = verify_equation_inventory_text(
            Path::new("validation/equation_inventory.tsv"),
            &minimal_equation_inventory_fixture(),
            &equation_inventory_expected_counts(),
            Some(&public_functions),
        )
        .expect_err("missing public function coverage should fail");
        assert!(err.contains("missing public function inventory row"));
    }

    fn status_vocabulary_fixture() -> String {
        "verification_status:\n  allowed_values:\n    - research_required\n    - equation_traceable\n    - implementation_verified\n    - reference_validated\n    - experiment_validated\ndata_validation_status:\n  allowed_values:\n    - registered_fixture_only\nhash_status:\n  allowed_values:\n    - sha256_verified\n".to_string()
    }

    #[test]
    fn valid_status_vocabulary_fixture_passes() {
        let vocabulary = status_vocabulary_fixture();
        let cards = [(
            "card.yaml",
            "id: example.card\nstatus: research_required\nsource:\n  status: equation_traceable\n",
        )];
        let sources = [(
            "source.yaml",
            "id: source.example.card\nstatus: research_required\n",
        )];
        let data_registry = minimal_data_registry_entry("artifact.status_fixture");
        verify_status_vocabulary_text(
            Path::new("status_vocabulary.yaml"),
            &vocabulary,
            &cards,
            &sources,
            Path::new("DATA_REGISTRY.yaml"),
            &data_registry,
        )
        .expect("fixture vocabulary should cover validation and data-registry statuses");
    }

    #[test]
    fn status_vocabulary_missing_verification_value_fails() {
        let vocabulary = status_vocabulary_fixture().replace("    - equation_traceable\n", "");
        let cards = [(
            "card.yaml",
            "id: example.card\nstatus: research_required\nsource:\n  status: equation_traceable\n",
        )];
        let sources = [(
            "source.yaml",
            "id: source.example.card\nstatus: research_required\n",
        )];
        let data_registry = minimal_data_registry_entry("artifact.status_fixture");
        let err = verify_status_vocabulary_text(
            Path::new("status_vocabulary.yaml"),
            &vocabulary,
            &cards,
            &sources,
            Path::new("DATA_REGISTRY.yaml"),
            &data_registry,
        )
        .expect_err("missing verification status should fail");
        assert!(err.contains("missing verification status `equation_traceable`"));
    }

    #[test]
    fn unknown_card_status_fails_status_vocabulary_check() {
        let vocabulary = status_vocabulary_fixture();
        let cards = [(
            "card.yaml",
            "id: example.card\nstatus: flight_ready\nsource:\n  status: research_required\n",
        )];
        let sources = [(
            "source.yaml",
            "id: source.example.card\nstatus: research_required\n",
        )];
        let data_registry = minimal_data_registry_entry("artifact.status_fixture");
        let err = verify_status_vocabulary_text(
            Path::new("status_vocabulary.yaml"),
            &vocabulary,
            &cards,
            &sources,
            Path::new("DATA_REGISTRY.yaml"),
            &data_registry,
        )
        .expect_err("unknown card status should fail");
        assert!(err.contains("card.yaml field `status` has unsupported status `flight_ready`"));
    }

    #[test]
    fn unknown_data_registry_validation_status_fails_status_vocabulary_check() {
        let vocabulary = status_vocabulary_fixture();
        let cards = [(
            "card.yaml",
            "id: example.card\nstatus: research_required\nsource:\n  status: research_required\n",
        )];
        let sources = [(
            "source.yaml",
            "id: source.example.card\nstatus: research_required\n",
        )];
        let data_registry = minimal_data_registry_entry("artifact.status_fixture").replace(
            "    validation_status: registered_fixture_only\n",
            "    validation_status: surprise_status\n",
        );
        let err = verify_status_vocabulary_text(
            Path::new("status_vocabulary.yaml"),
            &vocabulary,
            &cards,
            &sources,
            Path::new("DATA_REGISTRY.yaml"),
            &data_registry,
        )
        .expect_err("unknown data-registry status should fail");
        assert!(err.contains("data-registry entry `artifact.status_fixture` field `validation_status` has unsupported status `surprise_status`"));
    }

    #[test]
    fn readiness_claim_in_allowed_status_vocabulary_fails() {
        let vocabulary = status_vocabulary_fixture().replace(
            "    - experiment_validated\n",
            "    - experiment_validated\n    - certified\n",
        );
        let cards = [(
            "card.yaml",
            "id: example.card\nstatus: research_required\nsource:\n  status: research_required\n",
        )];
        let sources = [(
            "source.yaml",
            "id: source.example.card\nstatus: research_required\n",
        )];
        let data_registry = minimal_data_registry_entry("artifact.status_fixture");
        let err = verify_status_vocabulary_text(
            Path::new("status_vocabulary.yaml"),
            &vocabulary,
            &cards,
            &sources,
            Path::new("DATA_REGISTRY.yaml"),
            &data_registry,
        )
        .expect_err("certification/readiness vocabulary values should fail");
        assert!(err.contains("forbidden readiness status `certified`"));
    }
}
