pub mod check;
pub mod rust;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use crate::equation_batch::manifest::{parse_equation_batch_manifest, EquationBatchRow};

pub const REGISTRY_SCHEMA_VERSION: &str = "aerocodex.formula_registry.v1";
pub const GENERATOR_VERSION: &str = "xtask-formula-registry-v1";
pub const GENERATED_BY: &str = "cargo run -p xtask -- formula-registry generate";
pub const REGISTRY_JSON_PATH: &str = "generated/formula_registry.json";
pub const REGISTRY_SHA256_PATH: &str = "generated/formula_registry.sha256";
pub const SAFETY_NOTICE: &str = "AeroCodex Formula Registry v1 is deterministic research/preliminary-design inventory metadata only; it is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval, and it does not make formulas executable or promote validation status.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateOptions {
    pub out: PathBuf,
    pub check: bool,
}

impl GenerateOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut out: Option<PathBuf> = None;
        let mut check = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--out" => {
                    if out.is_some() {
                        return Err("usage error: --out was supplied more than once".to_string());
                    }
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "usage error: --out requires a path".to_string())?;
                    if value.starts_with("--") {
                        return Err("usage error: --out requires a path".to_string());
                    }
                    out = Some(PathBuf::from(value));
                }
                "--check" => {
                    if check {
                        return Err("usage error: --check was supplied more than once".to_string());
                    }
                    check = true;
                }
                unknown if unknown.starts_with("--") => {
                    return Err(format!(
                        "usage error: unknown formula-registry generate flag `{unknown}`"
                    ));
                }
                unexpected => {
                    return Err(format!(
                        "usage error: unexpected formula-registry generate argument `{unexpected}`"
                    ));
                }
            }
            index += 1;
        }

        let out = out.ok_or_else(|| {
            "usage error: formula-registry generate requires --out PATH".to_string()
        })?;

        Ok(Self { out, check })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormulaRegistry {
    pub schema_version: String,
    pub generator_version: String,
    pub generated_by: String,
    pub source_hash: String,
    pub formula_count: usize,
    pub non_claims: Vec<String>,
    pub formulas: Vec<FormulaEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormulaEntry {
    pub formula_id: String,
    pub legacy_formula_id: Option<String>,
    pub aliases: Vec<String>,
    pub name: String,
    pub summary: Option<String>,
    pub family: String,
    pub batch_id: Option<String>,
    pub status: String,
    pub quarantine_state: String,
    pub execution_policy: String,
    pub source_trace: BTreeMap<String, String>,
    pub inputs: Vec<BTreeMap<String, String>>,
    pub outputs: Vec<BTreeMap<String, String>>,
    pub units: Option<BTreeMap<String, String>>,
    pub domain_constraints: Vec<BTreeMap<String, String>>,
    pub implementation_path: BTreeMap<String, String>,
    pub runtime_symbol: Option<String>,
    pub test_vectors: Vec<BTreeMap<String, String>>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sidecar {
    path: String,
    formula_id: String,
    legacy_formula_id: Option<String>,
    name: Option<String>,
    family: Option<String>,
    inputs: Vec<BTreeMap<String, String>>,
    outputs: Vec<BTreeMap<String, String>>,
    units: Option<BTreeMap<String, String>>,
    domain_constraints: Vec<BTreeMap<String, String>>,
    warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AliasRecord {
    canonical_id: String,
    legacy_alias: Option<String>,
    name: Option<String>,
    inputs: Vec<BTreeMap<String, String>>,
    outputs: Vec<BTreeMap<String, String>>,
    units: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct AliasPolicy {
    by_canonical: BTreeMap<String, AliasRecord>,
    canonical_by_alias: BTreeMap<String, String>,
}

pub fn run_generate_command(root: &Path, options: &GenerateOptions) -> Result<(), String> {
    require_approved_output_path(&options.out)?;
    let registry = build_formula_registry(root)?;
    let expected = render_formula_registry_json(&registry);
    let expected_sha = render_formula_registry_sha256(&expected);
    let out_path = output_path(root, &options.out);
    let sha_path = root.join(REGISTRY_SHA256_PATH);

    if options.check {
        let existing = fs::read_to_string(&out_path).map_err(|error| {
            format!(
                "formula registry check failed; cannot read {}: {error}",
                out_path.display()
            )
        })?;
        if existing != expected {
            return Err(format!(
                "formula registry check failed; output is missing or stale: {}",
                out_path.display()
            ));
        }
        let existing_sha = fs::read_to_string(&sha_path).map_err(|error| {
            format!(
                "formula registry check failed; cannot read sha256 sidecar {}: {error}",
                sha_path.display()
            )
        })?;
        if existing_sha != expected_sha {
            return Err(format!(
                "formula registry check failed; sha256 sidecar is missing or stale: {}",
                sha_path.display()
            ));
        }
        println!(
            "formula_registry_check=PASS path={} sha256_path={}",
            out_path.display(),
            sha_path.display()
        );
        return Ok(());
    }

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    fs::write(&out_path, expected).map_err(|error| format!("{}: {error}", out_path.display()))?;
    fs::write(&sha_path, expected_sha)
        .map_err(|error| format!("{}: {error}", sha_path.display()))?;
    println!(
        "wrote_formula_registry={} wrote_formula_registry_sha256={} formula_count={} source_hash={}",
        out_path.display(),
        sha_path.display(),
        registry.formula_count,
        registry.source_hash
    );
    Ok(())
}

pub fn build_formula_registry(root: &Path) -> Result<FormulaRegistry, String> {
    let manifest_paths = collect_manifest_paths(root)?;
    let alias_policy = load_alias_policy(root)?;
    let sidecars = load_sidecars(root)?;
    let mut formulas = Vec::new();
    let mut seen_canonical = BTreeSet::new();

    for relative_path in &manifest_paths {
        let absolute_path = root.join(relative_path);
        let text = fs::read_to_string(&absolute_path)
            .map_err(|error| format!("{}: {error}", relative_path.display()))?;
        let manifest = parse_equation_batch_manifest(relative_path, &text)?;
        let mut rows: Vec<&EquationBatchRow> = manifest.rows.iter().collect();
        rows.sort_by(|left, right| {
            normalize_formula_id(&left.formula_id, &alias_policy)
                .cmp(&normalize_formula_id(&right.formula_id, &alias_policy))
                .then(left.formula_id.cmp(&right.formula_id))
                .then(left.line_number.cmp(&right.line_number))
        });

        for row in rows {
            let canonical_id = normalize_formula_id(&row.formula_id, &alias_policy);
            validate_canonical_id(&canonical_id).map_err(|error| {
                format!(
                    "{} line {} formula_id `{}` normalized to invalid canonical formula_id `{}`: {error}",
                    relative_path.display(),
                    row.line_number,
                    row.formula_id,
                    canonical_id
                )
            })?;
            if !seen_canonical.insert(canonical_id.clone()) {
                return Err(format!(
                    "duplicate canonical formula_id `{canonical_id}` from {} line {}",
                    relative_path.display(),
                    row.line_number
                ));
            }
            let sidecar = sidecar_for(&canonical_id, &row.formula_id, &sidecars)?;
            formulas.push(formula_entry_from_row(
                relative_path,
                row,
                &canonical_id,
                sidecar,
                &alias_policy,
            )?);
        }
    }

    formulas.sort_by(|left, right| left.formula_id.cmp(&right.formula_id));
    verify_alias_uniqueness(&formulas)?;

    let source_hash = source_hash(root, &manifest_paths, &alias_policy, &sidecars)?;
    let formula_count = formulas.len();

    Ok(FormulaRegistry {
        schema_version: REGISTRY_SCHEMA_VERSION.to_string(),
        generator_version: GENERATOR_VERSION.to_string(),
        generated_by: GENERATED_BY.to_string(),
        source_hash,
        formula_count,
        non_claims: non_claims(),
        formulas,
    })
}

fn formula_entry_from_row(
    manifest_path: &Path,
    row: &EquationBatchRow,
    canonical_id: &str,
    sidecar: Option<&Sidecar>,
    alias_policy: &AliasPolicy,
) -> Result<FormulaEntry, String> {
    let alias_record = alias_policy.by_canonical.get(canonical_id);
    let legacy_formula_id = primary_legacy_formula_id(row, sidecar, alias_record);
    let aliases = aliases_for(row, sidecar, alias_record, canonical_id);
    let family = sidecar
        .and_then(|sidecar| sidecar.family.clone())
        .unwrap_or_else(|| family_from_formula_id(canonical_id));
    let name = sidecar
        .and_then(|sidecar| sidecar.name.clone())
        .or_else(|| alias_record.and_then(|record| record.name.clone()))
        .unwrap_or_else(|| name_from_formula_id(canonical_id));

    let mut source_trace = BTreeMap::new();
    source_trace.insert("manifest_path".to_string(), path_string(manifest_path));
    source_trace.insert("manifest_line".to_string(), row.line_number.to_string());
    source_trace.insert("source_formula_id".to_string(), row.formula_id.clone());
    source_trace.insert("contract_path".to_string(), row.contract_path.clone());
    source_trace.insert(
        "validation_card_path".to_string(),
        row.validation_card_path.clone(),
    );
    source_trace.insert("source_seed_path".to_string(), row.source_seed_path.clone());
    if let Some(sidecar) = sidecar {
        source_trace.insert("sidecar_path".to_string(), sidecar.path.clone());
    }

    let mut implementation_path = BTreeMap::new();
    implementation_path.insert("package".to_string(), row.package.clone());
    implementation_path.insert("crate_name".to_string(), row.crate_name.clone());
    implementation_path.insert("runtime_symbol".to_string(), row.runtime_symbol.clone());
    implementation_path.insert("output_variable".to_string(), row.output_variable.clone());
    implementation_path.insert("test_strategy".to_string(), row.test_strategy.clone());
    implementation_path.insert("test_expression".to_string(), row.test_expression.clone());

    let inputs = sidecar
        .map(|sidecar| sidecar.inputs.clone())
        .filter(|inputs| !inputs.is_empty())
        .or_else(|| {
            alias_record
                .map(|record| record.inputs.clone())
                .filter(|inputs| !inputs.is_empty())
        })
        .unwrap_or_default();
    let outputs = sidecar
        .map(|sidecar| sidecar.outputs.clone())
        .filter(|outputs| !outputs.is_empty())
        .or_else(|| {
            alias_record
                .map(|record| record.outputs.clone())
                .filter(|outputs| !outputs.is_empty())
        })
        .unwrap_or_else(|| output_from_manifest(row));
    let units = sidecar
        .and_then(|sidecar| sidecar.units.clone())
        .or_else(|| alias_record.and_then(|record| record.units.clone()));
    let domain_constraints = sidecar
        .map(|sidecar| sidecar.domain_constraints.clone())
        .unwrap_or_default();

    let mut warnings = Vec::new();
    warnings.push(
        "Registry inclusion does not make formulas executable or promote validation status."
            .to_string(),
    );
    if row.validation_status == "research_required" {
        warnings.push(
            "research_required rows are inventory only and remain blocked by default execution gates."
                .to_string(),
        );
    }
    if legacy_formula_id.is_some() {
        warnings.push(
            "legacy_formula_id is preserved as an alias for traceability and does not bypass status gates."
                .to_string(),
        );
    }
    if sidecar.is_none() {
        warnings.push(
            "No formula sidecar metadata was found; this entry is generated from equation-batch TSV and governed alias data only."
                .to_string(),
        );
    }
    if let Some(sidecar) = sidecar {
        for warning in &sidecar.warnings {
            push_unique(&mut warnings, warning.clone());
        }
    }

    Ok(FormulaEntry {
        formula_id: canonical_id.to_string(),
        legacy_formula_id,
        aliases,
        name,
        summary: Some(
            "Generated Formula Registry v1 inventory entry for research/preliminary-design traceability; execution remains controlled by status gates."
                .to_string(),
        ),
        family,
        batch_id: Some(row.batch_id.clone()),
        status: schema_status(&row.validation_status)?.to_string(),
        quarantine_state: quarantine_state(canonical_id, &row.validation_status).to_string(),
        execution_policy: execution_policy(&row.validation_status)?.to_string(),
        source_trace,
        inputs,
        outputs,
        units,
        domain_constraints,
        implementation_path,
        runtime_symbol: Some(row.runtime_symbol.clone()),
        test_vectors: Vec::new(),
        warnings,
    })
}

fn schema_status(status: &str) -> Result<&'static str, String> {
    match status {
        "research_required" => Ok("research_required"),
        "equation_traceable" => Ok("equation_traceable"),
        "implementation_verified" => Ok("implementation_verified"),
        "reference_validated" => Ok("reference_validated"),
        "experiment_validated" => Err(
            "Formula Registry v1 does not accept experiment_validated status; later schema work must govern that promotion".to_string(),
        ),
        other => Err(format!("unsupported validation_status `{other}`")),
    }
}

fn execution_policy(status: &str) -> Result<&'static str, String> {
    match status {
        "research_required" => Ok("blocked"),
        "equation_traceable" => Ok("preliminary_flag_required"),
        "implementation_verified" => Ok("normal_research"),
        "reference_validated" => Ok("publication_supporting"),
        other => Err(format!("unsupported execution-policy status `{other}`")),
    }
}

fn quarantine_state(canonical_id: &str, status: &str) -> &'static str {
    if canonical_id.starts_with("m07.") {
        "m07_candidate_blocked"
    } else if matches!(status, "research_required" | "equation_traceable") {
        "below_execution_threshold"
    } else {
        "none"
    }
}

fn primary_legacy_formula_id(
    row: &EquationBatchRow,
    sidecar: Option<&Sidecar>,
    alias_record: Option<&AliasRecord>,
) -> Option<String> {
    if row.formula_id.starts_with("formula_vault.") {
        return Some(row.formula_id.clone());
    }
    sidecar
        .and_then(|sidecar| sidecar.legacy_formula_id.clone())
        .or_else(|| alias_record.and_then(|record| record.legacy_alias.clone()))
}

fn aliases_for(
    row: &EquationBatchRow,
    sidecar: Option<&Sidecar>,
    alias_record: Option<&AliasRecord>,
    canonical_id: &str,
) -> Vec<String> {
    let mut aliases = Vec::new();
    if row.formula_id != canonical_id {
        push_unique(&mut aliases, row.formula_id.clone());
    }
    if let Some(sidecar) = sidecar {
        if let Some(legacy) = &sidecar.legacy_formula_id {
            if legacy != canonical_id {
                push_unique(&mut aliases, legacy.clone());
            }
        }
    }
    if let Some(record) = alias_record {
        if let Some(legacy) = &record.legacy_alias {
            if legacy != canonical_id {
                push_unique(&mut aliases, legacy.clone());
            }
        }
    }
    aliases.sort();
    aliases
}

fn normalize_formula_id(raw: &str, alias_policy: &AliasPolicy) -> String {
    if let Some(canonical) = alias_policy.canonical_by_alias.get(raw) {
        return canonical.clone();
    }
    if let Some(remainder) = raw.strip_prefix("formula_vault.") {
        return normalize_formula_vault_remainder(remainder);
    }
    raw.to_string()
}

fn normalize_formula_vault_remainder(remainder: &str) -> String {
    let mut parts = remainder.split('.');
    let first = parts.next().unwrap_or_default();
    let mut canonical = first.replace('_', ".");
    for part in parts {
        canonical.push('.');
        canonical.push_str(part);
    }
    canonical
}

fn family_from_formula_id(formula_id: &str) -> String {
    let parts: Vec<&str> = formula_id.split('.').collect();
    if parts.len() >= 2 {
        format!("{}.{}", parts[0], parts[1])
    } else {
        formula_id.to_string()
    }
}

fn name_from_formula_id(formula_id: &str) -> String {
    let leaf = formula_id.rsplit('.').next().unwrap_or(formula_id);
    leaf.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn output_from_manifest(row: &EquationBatchRow) -> Vec<BTreeMap<String, String>> {
    let mut output = BTreeMap::new();
    output.insert("name".to_string(), row.output_variable.clone());
    output.insert("type".to_string(), "f64".to_string());
    vec![output]
}

fn load_alias_policy(root: &Path) -> Result<AliasPolicy, String> {
    let path = root.join("docs/registry/m00_formula_id_map.csv");
    if !path.is_file() {
        return Ok(AliasPolicy::default());
    }
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut policy = AliasPolicy::default();
    for (index, line) in text.lines().enumerate() {
        let line_no = index + 1;
        if line_no == 1 || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split(',').map(str::trim).collect();
        if fields.len() < 12 {
            return Err(format!(
                "docs/registry/m00_formula_id_map.csv line {line_no} has {} fields; expected at least 12",
                fields.len()
            ));
        }
        let canonical_id = fields[0].to_string();
        let legacy_alias = empty_to_none(fields[1]);
        validate_canonical_id(&canonical_id).map_err(|error| {
            format!(
                "docs/registry/m00_formula_id_map.csv line {line_no} canonical_id `{canonical_id}` is invalid: {error}"
            )
        })?;
        if policy.by_canonical.contains_key(&canonical_id) {
            return Err(format!(
                "duplicate canonical formula_id `{canonical_id}` in docs/registry/m00_formula_id_map.csv line {line_no}"
            ));
        }
        if let Some(alias) = &legacy_alias {
            if let Some(existing) = policy.canonical_by_alias.get(alias) {
                return Err(format!(
                    "duplicate alias `{alias}` maps to both `{existing}` and `{canonical_id}` in docs/registry/m00_formula_id_map.csv line {line_no}"
                ));
            }
            policy
                .canonical_by_alias
                .insert(alias.clone(), canonical_id.clone());
        }
        let inputs = parse_field_specs(fields[4]);
        let outputs = parse_field_specs(fields[5]);
        let units = units_from_specs(&inputs, &outputs);
        policy.by_canonical.insert(
            canonical_id.clone(),
            AliasRecord {
                canonical_id,
                legacy_alias,
                name: empty_to_none(fields[2]),
                inputs,
                outputs,
                units,
            },
        );
    }
    Ok(policy)
}

fn parse_field_specs(value: &str) -> Vec<BTreeMap<String, String>> {
    value
        .split(';')
        .flat_map(|chunk| chunk.split('|'))
        .filter_map(|spec| {
            let spec = spec.trim();
            if spec.is_empty() {
                return None;
            }
            let parts: Vec<&str> = spec.split(':').collect();
            if parts.is_empty() || parts[0].trim().is_empty() {
                return None;
            }
            let mut object = BTreeMap::new();
            object.insert("name".to_string(), parts[0].trim().to_string());
            if let Some(kind) = parts
                .get(1)
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
            {
                object.insert("type".to_string(), kind.to_string());
            }
            if let Some(unit) = parts
                .get(2)
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
            {
                object.insert("unit".to_string(), unit.to_string());
            }
            object.insert("required".to_string(), "true".to_string());
            Some(object)
        })
        .collect()
}

fn units_from_specs(
    inputs: &[BTreeMap<String, String>],
    outputs: &[BTreeMap<String, String>],
) -> Option<BTreeMap<String, String>> {
    let mut units = BTreeMap::new();
    if let Some(input_unit) = inputs.first().and_then(|input| input.get("unit")) {
        units.insert("input".to_string(), input_unit.clone());
    }
    if let Some(output_unit) = outputs.first().and_then(|output| output.get("unit")) {
        units.insert("output".to_string(), output_unit.clone());
    }
    if units.is_empty() {
        None
    } else {
        Some(units)
    }
}

fn load_sidecars(root: &Path) -> Result<Vec<Sidecar>, String> {
    let sidecar_dir = root.join("formula-schemas");
    if !sidecar_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    collect_yaml_paths(&sidecar_dir, &mut paths)?;
    paths.sort();

    let mut sidecars = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let mut seen_legacy = BTreeSet::new();
    for path in paths {
        let relative_path = relative_path(root, &path)?;
        let text =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let sidecar = parse_sidecar(&relative_path, &text)?;
        validate_canonical_id(&sidecar.formula_id).map_err(|error| {
            format!(
                "{} formula_id `{}` is invalid: {error}",
                relative_path, sidecar.formula_id
            )
        })?;
        if !seen_ids.insert(sidecar.formula_id.clone()) {
            return Err(format!(
                "duplicate sidecar formula_id `{}` at {}",
                sidecar.formula_id, relative_path
            ));
        }
        if let Some(legacy) = &sidecar.legacy_formula_id {
            if !seen_legacy.insert(legacy.clone()) {
                return Err(format!(
                    "duplicate sidecar legacy_formula_id `{legacy}` at {relative_path}"
                ));
            }
        }
        sidecars.push(sidecar);
    }
    Ok(sidecars)
}

fn collect_yaml_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_yaml_paths(&path, paths)?;
        } else if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "yaml" | "yml"))
        {
            paths.push(path);
        }
    }
    Ok(())
}

fn parse_sidecar(path: &str, text: &str) -> Result<Sidecar, String> {
    let mut formula_id = None;
    let mut legacy_formula_id = None;
    let mut name = None;
    let mut family = None;
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut units: Option<BTreeMap<String, String>> = None;
    let mut domain_constraints = Vec::new();
    let mut warnings = Vec::new();
    let mut section: Option<&str> = None;

    for (index, raw_line) in text.lines().enumerate() {
        let line_no = index + 1;
        let line_without_comment = raw_line.split_once('#').map_or(raw_line, |(head, _)| head);
        let trimmed = line_without_comment.trim_end();
        if trimmed.trim().is_empty() {
            continue;
        }
        let leading = trimmed.len() - trimmed.trim_start().len();
        let content = trimmed.trim_start();
        if leading == 0 {
            if content.ends_with(':') {
                section = Some(content.trim_end_matches(':'));
                continue;
            }
            section = None;
            if let Some((key, value)) = content.split_once(':') {
                let value = clean_yaml_scalar(value);
                match key.trim() {
                    "formula_id" => formula_id = Some(value),
                    "legacy_formula_id" => legacy_formula_id = empty_to_none(&value),
                    "name" => name = empty_to_none(&value),
                    "family" => family = empty_to_none(&value),
                    _ => {}
                }
            }
            continue;
        }

        match section {
            Some("inputs") => parse_yaml_object_line(path, line_no, content, &mut inputs)?,
            Some("outputs") => parse_yaml_object_line(path, line_no, content, &mut outputs)?,
            Some("domain_constraints") => {
                parse_yaml_object_line(path, line_no, content, &mut domain_constraints)?;
            }
            Some("units") => {
                if let Some((key, value)) = content.split_once(':') {
                    let units = units.get_or_insert_with(BTreeMap::new);
                    units.insert(key.trim().to_string(), clean_yaml_scalar(value));
                }
            }
            Some("warnings") => {
                if let Some(item) = content.strip_prefix("- ") {
                    warnings.push(clean_yaml_scalar(item));
                }
            }
            _ => {}
        }
    }

    let formula_id = formula_id.ok_or_else(|| format!("{path}: sidecar missing formula_id"))?;
    Ok(Sidecar {
        path: path.to_string(),
        formula_id,
        legacy_formula_id,
        name,
        family,
        inputs,
        outputs,
        units,
        domain_constraints,
        warnings,
    })
}

fn parse_yaml_object_line(
    path: &str,
    line_no: usize,
    content: &str,
    objects: &mut Vec<BTreeMap<String, String>>,
) -> Result<(), String> {
    if let Some(item) = content.strip_prefix("- ") {
        let mut object = BTreeMap::new();
        if let Some((key, value)) = item.split_once(':') {
            object.insert(key.trim().to_string(), clean_yaml_scalar(value));
        } else if !item.trim().is_empty() {
            return Err(format!(
                "{path} line {line_no}: unsupported YAML list item `{item}`"
            ));
        }
        objects.push(object);
    } else if let Some((key, value)) = content.split_once(':') {
        let object = objects
            .last_mut()
            .ok_or_else(|| format!("{path} line {line_no}: YAML field without list item"))?;
        let key = key.trim();
        if matches!(key, "finite" | "min" | "max") {
            return Ok(());
        }
        object.insert(key.to_string(), clean_yaml_scalar(value));
    }
    Ok(())
}

fn sidecar_for<'a>(
    canonical_id: &str,
    source_formula_id: &str,
    sidecars: &'a [Sidecar],
) -> Result<Option<&'a Sidecar>, String> {
    let mut matches = Vec::new();
    for sidecar in sidecars {
        if sidecar.formula_id == canonical_id
            || sidecar.legacy_formula_id.as_deref() == Some(source_formula_id)
        {
            matches.push(sidecar);
        }
    }
    if matches.len() > 1 {
        return Err(format!(
            "ambiguous sidecar match for canonical formula_id `{canonical_id}` / source `{source_formula_id}`"
        ));
    }
    Ok(matches.into_iter().next())
}

fn collect_manifest_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let batch_dir = root.join("equation-batches");
    if !batch_dir.is_dir() {
        return Err("equation-batches directory is missing".to_string());
    }
    let entries = fs::read_dir(&batch_dir).map_err(|error| format!("equation-batches: {error}"))?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("equation-batches: {error}"))?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension == "tsv")
        {
            paths.push(relative_path(root, &path).map(PathBuf::from)?);
        }
    }
    paths.sort();
    if paths.is_empty() {
        return Err("equation-batches has no TSV manifests".to_string());
    }
    Ok(paths)
}

fn verify_alias_uniqueness(formulas: &[FormulaEntry]) -> Result<(), String> {
    let canonical_ids: BTreeSet<&str> = formulas
        .iter()
        .map(|formula| formula.formula_id.as_str())
        .collect();
    let mut alias_to_canonical: BTreeMap<&str, &str> = BTreeMap::new();
    for formula in formulas {
        for alias in &formula.aliases {
            if canonical_ids.contains(alias.as_str()) {
                return Err(format!(
                    "ambiguous alias conflict: alias `{alias}` is also a canonical formula_id"
                ));
            }
            match alias_to_canonical.insert(alias.as_str(), formula.formula_id.as_str()) {
                Some(existing) if existing != formula.formula_id => {
                    return Err(format!(
                        "duplicate alias `{alias}` maps to both `{existing}` and `{}`",
                        formula.formula_id
                    ));
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn source_hash(
    root: &Path,
    manifest_paths: &[PathBuf],
    alias_policy: &AliasPolicy,
    sidecars: &[Sidecar],
) -> Result<String, String> {
    let mut inputs = Vec::new();
    for path in manifest_paths {
        inputs.push(path.clone());
    }
    if !alias_policy.by_canonical.is_empty() {
        inputs.push(PathBuf::from("docs/registry/m00_formula_id_map.csv"));
    }
    for sidecar in sidecars {
        inputs.push(PathBuf::from(&sidecar.path));
    }
    inputs.sort();
    inputs.dedup();

    let mut material = String::new();
    material.push_str("aerocodex.formula_registry.source_hash.v1\n");
    for relative in inputs {
        let text = fs::read_to_string(root.join(&relative))
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        writeln!(&mut material, "path:{}", path_string(&relative)).expect("write to string");
        writeln!(&mut material, "len:{}", text.len()).expect("write to string");
        material.push_str(&text);
        if !text.ends_with('\n') {
            material.push('\n');
        }
        material.push_str("--end-input--\n");
    }
    Ok(format!(
        "sha256:{}",
        crate::equation_batch::generate::sha256_hex(material.as_bytes())
    ))
}

pub fn render_formula_registry_json(registry: &FormulaRegistry) -> String {
    let mut out = String::new();
    out.push('{');
    push_string_field(
        &mut out,
        2,
        "schema_version",
        &registry.schema_version,
        true,
    );
    push_string_field(
        &mut out,
        2,
        "generator_version",
        &registry.generator_version,
        true,
    );
    push_string_field(&mut out, 2, "generated_by", &registry.generated_by, true);
    push_string_field(&mut out, 2, "source_hash", &registry.source_hash, true);
    push_usize_field(&mut out, 2, "formula_count", registry.formula_count, true);
    push_string_array_field(&mut out, 2, "non_claims", &registry.non_claims, true);
    push_formulas_field(&mut out, &registry.formulas, false);
    out.push_str("\n}\n");
    out
}

fn push_formulas_field(out: &mut String, formulas: &[FormulaEntry], comma: bool) {
    let suffix = if comma { "," } else { "" };
    out.push_str("\n  \"formulas\": [");
    for (index, formula) in formulas.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_formula(out, formula, 4);
    }
    out.push_str("\n  ]");
    out.push_str(suffix);
}

fn push_formula(out: &mut String, formula: &FormulaEntry, indent: usize) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    push_string_field(out, indent + 2, "formula_id", &formula.formula_id, true);
    push_optional_string_field(
        out,
        indent + 2,
        "legacy_formula_id",
        formula.legacy_formula_id.as_deref(),
        true,
    );
    push_string_array_field(out, indent + 2, "aliases", &formula.aliases, true);
    push_string_field(out, indent + 2, "name", &formula.name, true);
    push_optional_string_field(out, indent + 2, "summary", formula.summary.as_deref(), true);
    push_string_field(out, indent + 2, "family", &formula.family, true);
    push_optional_string_field(
        out,
        indent + 2,
        "batch_id",
        formula.batch_id.as_deref(),
        true,
    );
    push_string_field(out, indent + 2, "status", &formula.status, true);
    push_string_field(
        out,
        indent + 2,
        "quarantine_state",
        &formula.quarantine_state,
        true,
    );
    push_string_field(
        out,
        indent + 2,
        "execution_policy",
        &formula.execution_policy,
        true,
    );
    push_string_map_field(out, indent + 2, "source_trace", &formula.source_trace, true);
    push_object_array_field(out, indent + 2, "inputs", &formula.inputs, true);
    push_object_array_field(out, indent + 2, "outputs", &formula.outputs, true);
    push_optional_string_map_field(out, indent + 2, "units", formula.units.as_ref(), true);
    push_object_array_field(
        out,
        indent + 2,
        "domain_constraints",
        &formula.domain_constraints,
        true,
    );
    push_string_map_field(
        out,
        indent + 2,
        "implementation_path",
        &formula.implementation_path,
        true,
    );
    push_optional_string_field(
        out,
        indent + 2,
        "runtime_symbol",
        formula.runtime_symbol.as_deref(),
        true,
    );
    push_object_array_field(out, indent + 2, "test_vectors", &formula.test_vectors, true);
    push_string_array_field(out, indent + 2, "warnings", &formula.warnings, false);
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_string_field(out: &mut String, indent: usize, key: &str, value: &str, comma: bool) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
    write_json_string(out, value);
    push_suffix(out, comma);
}

fn push_optional_string_field(
    out: &mut String,
    indent: usize,
    key: &str,
    value: Option<&str>,
    comma: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
    if let Some(value) = value {
        write_json_string(out, value);
    } else {
        out.push_str("null");
    }
    push_suffix(out, comma);
}

fn push_usize_field(out: &mut String, indent: usize, key: &str, value: usize, comma: bool) {
    write_indent(out, indent);
    write_json_string(out, key);
    writeln!(out, ": {value}{}", if comma { "," } else { "" }).expect("write to string");
}

fn push_string_array_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &[String],
    comma: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": [");
    if values.is_empty() {
        out.push(']');
        push_suffix(out, comma);
        return;
    }
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('\n');
        write_indent(out, indent + 2);
        write_json_string(out, value);
    }
    out.push('\n');
    write_indent(out, indent);
    out.push(']');
    push_suffix(out, comma);
}

fn push_string_map_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &BTreeMap<String, String>,
    comma: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
    push_string_map_value(out, indent, values);
    push_suffix(out, comma);
}

fn push_optional_string_map_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: Option<&BTreeMap<String, String>>,
    comma: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
    if let Some(values) = values {
        push_string_map_value(out, indent, values);
    } else {
        out.push_str("null");
    }
    push_suffix(out, comma);
}

fn push_object_array_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &[BTreeMap<String, String>],
    comma: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": [");
    if values.is_empty() {
        out.push(']');
        push_suffix(out, comma);
        return;
    }
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('\n');
        write_indent(out, indent + 2);
        push_string_map_value(out, indent + 2, value);
    }
    out.push('\n');
    write_indent(out, indent);
    out.push(']');
    push_suffix(out, comma);
}

fn push_string_map_value(out: &mut String, indent: usize, values: &BTreeMap<String, String>) {
    out.push('{');
    if values.is_empty() {
        out.push('}');
        return;
    }
    for (index, (key, value)) in values.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('\n');
        write_indent(out, indent + 2);
        write_json_string(out, key);
        out.push_str(": ");
        if is_json_bool_key(key) && matches!(value.as_str(), "true" | "false") {
            out.push_str(value);
        } else {
            write_json_string(out, value);
        }
    }
    out.push('\n');
    write_indent(out, indent);
    out.push('}');
}

fn is_json_bool_key(key: &str) -> bool {
    matches!(key, "required" | "finite")
}

fn push_suffix(out: &mut String, comma: bool) {
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push(' ');
    }
}

fn write_json_string(out: &mut String, value: &str) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            character if character.is_control() => {
                write!(out, "\\u{:04x}", character as u32).expect("write to string");
            }
            character => out.push(character),
        }
    }
    out.push('"');
}

fn render_formula_registry_sha256(json: &str) -> String {
    format!(
        "{}  {}\n",
        crate::equation_batch::generate::sha256_hex(json.as_bytes()),
        REGISTRY_JSON_PATH
    )
}

fn require_approved_output_path(out: &Path) -> Result<(), String> {
    if out.is_absolute() || path_string(out) != REGISTRY_JSON_PATH {
        return Err(format!(
            "formula-registry generate must write the approved registry path `{REGISTRY_JSON_PATH}`"
        ));
    }
    Ok(())
}

fn output_path(root: &Path, out: &Path) -> PathBuf {
    if out.is_absolute() {
        out.to_path_buf()
    } else {
        root.join(out)
    }
}

fn relative_path(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map(path_string)
        .map_err(|error| format!("{}: {error}", path.display()))
}

fn path_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn clean_yaml_scalar(value: &str) -> String {
    let value = value.trim();
    let value = value.strip_prefix('"').unwrap_or(value);
    let value = value.strip_suffix('"').unwrap_or(value);
    let value = value.strip_prefix('\'').unwrap_or(value);
    let value = value.strip_suffix('\'').unwrap_or(value);
    value.to_string()
}

fn empty_to_none(value: &str) -> Option<String> {
    let value = clean_yaml_scalar(value);
    if value.is_empty() || value == "null" {
        None
    } else {
        Some(value)
    }
}

fn validate_canonical_id(value: &str) -> Result<(), String> {
    let mut segments = value.split('.');
    let first = segments
        .next()
        .ok_or_else(|| "missing first segment".to_string())?;
    if first.is_empty()
        || !first
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
    {
        return Err(
            "first segment must contain only lowercase ASCII letters or digits".to_string(),
        );
    }
    let mut segment_count = 1usize;
    for segment in segments {
        segment_count += 1;
        if segment.is_empty()
            || !segment.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
            })
        {
            return Err(
                "subsequent segments must contain only lowercase ASCII letters, digits, or underscores"
                    .to_string(),
            );
        }
    }
    if segment_count < 2 {
        return Err("canonical IDs must be dotted".to_string());
    }
    Ok(())
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

fn non_claims() -> Vec<String> {
    vec![
        "Schema validation is not formula validation.".to_string(),
        "Registry inclusion is not execution readiness and does not make formulas executable.".to_string(),
        "Research/preliminary-design software; not certified as operational aerospace software.".to_string(),
        "source_hash is deterministic build-input evidence, not a certification claim.".to_string(),
        "legacy_formula_id and aliases preserve traceability and do not bypass status gates.".to_string(),
        "M07 candidates remain blocked unless later promoted through explicit governed tasks.".to_string(),
        "The generator does not execute formulas, generate runtime dispatch, or promote validation status.".to_string(),
        SAFETY_NOTICE.to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::{
        build_formula_registry, render_formula_registry_json, run_generate_command, GenerateOptions,
    };

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    const HEADER: &str = "schema_version\tbatch_id\tformula_id\tpackage\tcrate_name\truntime_symbol\toutput_variable\tcontract_path\tvalidation_card_path\tsource_seed_path\tvalidation_status\ttest_strategy\ttest_expression";

    #[test]
    fn parse_args_requires_out_and_supports_check() {
        let options = GenerateOptions::parse_args(&["--out", "generated/formula_registry.json"])
            .expect("valid formula-registry args parse");
        assert_eq!(
            options.out.to_string_lossy(),
            "generated/formula_registry.json"
        );
        assert!(!options.check);

        let check_options =
            GenerateOptions::parse_args(&["--out", "generated/formula_registry.json", "--check"])
                .expect("valid formula-registry check args parse");
        assert!(check_options.check);

        let missing_out = GenerateOptions::parse_args(&[]).expect_err("missing out fails");
        assert!(missing_out.contains("usage error"));
        assert!(missing_out.contains("--out"));

        let duplicate_out = GenerateOptions::parse_args(&["--out", "a", "--out", "b"])
            .expect_err("duplicate out fails");
        assert!(duplicate_out.contains("usage error"));
        assert!(duplicate_out.contains("--out"));

        let unknown = GenerateOptions::parse_args(&["--out", "a", "--unknown"])
            .expect_err("unknown flag fails");
        assert!(unknown.contains("usage error"));
        assert!(unknown.contains("unknown"));
    }

    #[test]
    fn builds_registry_from_manifests_sidecars_and_alias_map() {
        let root = fake_repo_root("builds_registry");
        write_fixture_repo(&root);

        let registry = build_formula_registry(&root).expect("registry builds");
        assert_eq!(registry.schema_version, "aerocodex.formula_registry.v1");
        assert_eq!(registry.generator_version, "xtask-formula-registry-v1");
        assert_eq!(
            registry.generated_by,
            "cargo run -p xtask -- formula-registry generate"
        );
        assert_eq!(registry.formula_count, 2);
        assert_eq!(registry.source_hash.len(), "sha256:".len() + 64);
        assert_eq!(registry.formulas[0].formula_id, "m00.angle.deg_to_rad");
        assert_eq!(registry.formulas[1].formula_id, "m00.angle.rad_to_deg");

        let deg = &registry.formulas[0];
        assert_eq!(
            deg.legacy_formula_id.as_deref(),
            Some("formula_vault.m00.angle.deg2rad")
        );
        assert_eq!(deg.aliases, vec!["formula_vault.m00.angle.deg2rad"]);
        assert_eq!(deg.name, "Degrees to radians");
        assert_eq!(deg.family, "m00.angle");
        assert_eq!(deg.status, "research_required");
        assert_eq!(deg.quarantine_state, "below_execution_threshold");
        assert_eq!(deg.execution_policy, "blocked");
        assert_eq!(
            deg.source_trace.get("contract_path").map(String::as_str),
            Some("formula-vault/contracts/m00_angle_unit_conversions_contract.yaml")
        );
        assert_eq!(
            deg.source_trace
                .get("validation_card_path")
                .map(String::as_str),
            Some("validation/cards/validation_formula_vault_m00_angle_unit_conversions.yaml")
        );
        assert_eq!(
            deg.source_trace.get("source_seed_path").map(String::as_str),
            Some("validation/source_registry/source_formula_vault_m00_angle_unit_conversions.yaml")
        );
        assert_eq!(
            deg.source_trace.get("sidecar_path").map(String::as_str),
            Some("formula-schemas/m00/angle/deg_to_rad.yaml")
        );
        assert_eq!(
            deg.implementation_path.get("package").map(String::as_str),
            Some("aero-codex-astrodynamics")
        );
        assert_eq!(
            deg.implementation_path
                .get("crate_name")
                .map(String::as_str),
            Some("aero_codex_astrodynamics")
        );
        assert_eq!(
            deg.implementation_path
                .get("runtime_symbol")
                .map(String::as_str),
            Some("m00_degrees_to_radians")
        );
        assert_eq!(
            deg.inputs[0].get("name").map(String::as_str),
            Some("degrees")
        );
        assert_eq!(
            deg.outputs[0].get("name").map(String::as_str),
            Some("angle_radians")
        );
        assert!(deg
            .warnings
            .iter()
            .any(|warning| warning.contains("does not make formulas executable")));

        let rad = &registry.formulas[1];
        assert_eq!(
            rad.legacy_formula_id.as_deref(),
            Some("formula_vault.m00.angle.rad2deg")
        );
        assert_eq!(rad.aliases, vec!["formula_vault.m00.angle.rad2deg"]);
        assert_eq!(rad.execution_policy, "blocked");

        let json = render_formula_registry_json(&registry);
        assert!(json.contains("\"source_hash\": \"sha256:"));
        assert!(json.contains("formula_vault.m00.angle.deg2rad"));
        assert!(json.contains("\"execution_policy\": \"blocked\""));
        assert!(!json.to_lowercase().contains("wall-clock"));
        assert_eq!(json, render_formula_registry_json(&registry));

        remove_dir_if_exists(&root);
    }

    #[test]
    fn generate_command_writes_and_check_detects_stale_output() {
        let root = fake_repo_root("generate_check");
        write_fixture_repo(&root);
        let out = PathBuf::from("generated/formula_registry.json");
        let options = GenerateOptions {
            out: out.clone(),
            check: false,
        };

        run_generate_command(&root, &options).expect("generation succeeds");
        let out_path = root.join(&out);
        let sha_path = root.join("generated/formula_registry.sha256");
        let generated = fs::read_to_string(&out_path).expect("generated registry readable");
        let generated_sha = crate::equation_batch::generate::sha256_hex(generated.as_bytes());
        let sha_text =
            fs::read_to_string(&sha_path).expect("generated registry SHA sidecar readable");
        assert!(generated.contains("m00.angle.deg_to_rad"));
        assert_eq!(
            sha_text,
            format!("{generated_sha}  generated/formula_registry.json\n")
        );

        let check_options = GenerateOptions {
            out: out.clone(),
            check: true,
        };
        run_generate_command(&root, &check_options).expect("fresh check succeeds");

        fs::write(&sha_path, "0000000000000000000000000000000000000000000000000000000000000000  generated/formula_registry.json\n")
            .expect("write stale SHA sidecar");
        let stale_sha =
            run_generate_command(&root, &check_options).expect_err("stale SHA check fails");
        assert!(stale_sha.contains("formula registry check failed"));
        assert!(stale_sha.contains("sha256"));
        fs::write(&sha_path, &sha_text).expect("restore SHA sidecar");

        fs::remove_file(&sha_path).expect("remove SHA sidecar");
        let missing_sha =
            run_generate_command(&root, &check_options).expect_err("missing SHA check fails");
        assert!(missing_sha.contains("formula registry check failed"));
        assert!(missing_sha.contains("sha256"));
        fs::write(&sha_path, &sha_text).expect("restore SHA sidecar");

        fs::write(&out_path, "{\n  \"stale\": true\n}\n").expect("write stale registry");
        let stale = run_generate_command(&root, &check_options).expect_err("stale check fails");
        assert!(stale.contains("formula registry check failed"));
        assert!(stale.contains("stale"));

        remove_dir_if_exists(&root);
    }

    #[test]
    fn duplicate_canonical_ids_and_aliases_fail_closed() {
        let duplicate_canonical_root = fake_repo_root("duplicate_canonical");
        write_fixture_repo(&duplicate_canonical_root);
        append_manifest_row(
            &duplicate_canonical_root,
            "formula_vault.m00.angle.deg2rad",
            "m00_duplicate_degrees_to_radians",
        );
        let duplicate_canonical = build_formula_registry(&duplicate_canonical_root)
            .expect_err("duplicate canonical ID fails");
        assert!(duplicate_canonical.contains("duplicate canonical formula_id"));
        remove_dir_if_exists(&duplicate_canonical_root);

        let duplicate_alias_root = fake_repo_root("duplicate_alias");
        write_fixture_repo(&duplicate_alias_root);
        fs::write(
            duplicate_alias_root.join("docs/registry/m00_formula_id_map.csv"),
            "canonical_id,legacy_alias,name,input_flags,inputs,output,slice,first_slice_scope,normal_execution_policy,sidecar_template,promotion_packet_required,status_change_allowed_by_sidecar\n\
             m00.angle.deg_to_rad,formula_vault.m00.angle.same_alias,Degrees to radians,--degrees,degrees:f64:deg,angle_radians:f64:rad,A,fixture,requires_implementation_verified,template,yes,no promotion\n\
             m00.angle.rad_to_deg,formula_vault.m00.angle.same_alias,Radians to degrees,--radians,radians:f64:rad,angle_degrees:f64:deg,A,fixture,requires_implementation_verified,template,yes,no promotion\n",
        )
        .expect("write duplicate alias map");
        let duplicate_alias =
            build_formula_registry(&duplicate_alias_root).expect_err("duplicate alias fails");
        assert!(duplicate_alias.contains("duplicate alias"));
        remove_dir_if_exists(&duplicate_alias_root);
    }

    fn write_fixture_repo(root: &Path) {
        fs::create_dir_all(root.join("equation-batches")).expect("equation-batches dir");
        fs::create_dir_all(root.join("formula-schemas/m00/angle")).expect("formula-schemas dir");
        fs::create_dir_all(root.join("docs/registry")).expect("registry docs dir");
        fs::write(
            root.join("equation-batches/m00-angle-vector.tsv"),
            manifest_text(),
        )
        .expect("manifest write");
        fs::write(
            root.join("formula-schemas/m00/angle/deg_to_rad.yaml"),
            deg_to_rad_sidecar(),
        )
        .expect("sidecar write");
        fs::write(
            root.join("docs/registry/m00_formula_id_map.csv"),
            "canonical_id,legacy_alias,name,input_flags,inputs,output,slice,first_slice_scope,normal_execution_policy,sidecar_template,promotion_packet_required,status_change_allowed_by_sidecar\n\
             m00.angle.deg_to_rad,formula_vault.m00.angle.deg2rad,Degrees to radians,--degrees,degrees:f64:deg,angle_radians:f64:rad,A,fixture,requires_implementation_verified,template,yes,no promotion\n\
             m00.angle.rad_to_deg,formula_vault.m00.angle.rad2deg,Radians to degrees,--radians,radians:f64:rad,angle_degrees:f64:deg,A,fixture,requires_implementation_verified,template,yes,no promotion\n",
        )
        .expect("alias map write");
    }

    fn manifest_text() -> String {
        format!(
            "{HEADER}\n{}\n{}\n",
            manifest_row("formula_vault.m00.angle.deg2rad", "m00_degrees_to_radians"),
            manifest_row("formula_vault.m00.angle.rad2deg", "m00_radians_to_degrees"),
        )
    }

    fn append_manifest_row(root: &Path, formula_id: &str, runtime_symbol: &str) {
        let path = root.join("equation-batches/m00-angle-vector.tsv");
        let mut text = fs::read_to_string(&path).expect("manifest readable");
        text.push_str(&manifest_row(formula_id, runtime_symbol));
        text.push('\n');
        fs::write(path, text).expect("append manifest row");
    }

    fn manifest_row(formula_id: &str, runtime_symbol: &str) -> String {
        format!(
            "aerocodex.equation_batch.v1\tm00-slice-a-fixture\t{formula_id}\taero-codex-astrodynamics\taero_codex_astrodynamics\t{runtime_symbol}\tangle_radians\tformula-vault/contracts/m00_angle_unit_conversions_contract.yaml\tvalidation/cards/validation_formula_vault_m00_angle_unit_conversions.yaml\tvalidation/source_registry/source_formula_vault_m00_angle_unit_conversions.yaml\tresearch_required\ttolerance\tangle conversion fixture"
        )
    }

    fn deg_to_rad_sidecar() -> &'static str {
        "schema_version: aerocodex.formula_sidecar.v1\n\
         formula_id: m00.angle.deg_to_rad\n\
         legacy_formula_id: formula_vault.m00.angle.deg2rad\n\
         name: Degrees to radians\n\
         family: m00.angle\n\
         inputs:\n\
           - name: degrees\n\
             type: f64\n\
             unit: deg\n\
             required: true\n\
         outputs:\n\
           - name: angle_radians\n\
             type: f64\n\
             unit: rad\n\
         units:\n\
           input: deg\n\
           output: rad\n\
         domain_constraints:\n\
           - id: finite_input\n\
             expression: is_finite(degrees)\n\
         warnings:\n\
           - Fixture only; does not make formulas executable.\n"
    }

    fn fake_repo_root(label: &str) -> PathBuf {
        let count = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "aerocodex_rr015_formula_registry_{label}_{}_{}",
            std::process::id(),
            count
        ));
        remove_dir_if_exists(&path);
        fs::create_dir_all(&path).expect("create fake repo root");
        path
    }

    fn remove_dir_if_exists(path: &Path) {
        match fs::remove_dir_all(path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("{}: {error}", path.display()),
        }
    }
}
