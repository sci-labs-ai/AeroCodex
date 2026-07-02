use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    ffi::OsString,
    fmt::Write as _,
    fs,
    path::{Component, Path, PathBuf},
};

use super::{
    manifest::{parse_equation_batch_manifest, EquationBatchManifest, EquationBatchRow},
    plan,
};

pub const PROBE_SCHEMA_VERSION: &str = "aerocodex.equation_batch.probe_manifest_summary.v1";
const GENERATED_BY: &str = "xtask equation-batch generate";
pub const MARKER_FILE: &str = ".aerocodex-equation-batch-probe";
const REPO_LINK_DIR: &str = ".aerocodex-repo";
const RUST_CHECK_FILE: &str = "src/lib.rs";
const SUMMARY_FILE: &str = "manifest_summary.json";
const SAFETY_NOTICE: &str = "Temporary probe crate generation only: this command writes compiler-check scaffolding for manifest test expressions but does not run cargo test, execute formulas, generate registries, promote validation status, or change product CLI behavior.";
const NON_CLAIMS: &[&str] = &[
    "Generated probe files are not validation evidence by themselves.",
    "No certification, operational-use, regulated-use, flight-readiness, mission-readiness, habitat, or life-support approval is claimed.",
    "Formula execution and status promotion remain owned by later explicit RR tasks.",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateOptions {
    pub manifest: PathBuf,
    pub output_dir: PathBuf,
    pub json: bool,
}

impl GenerateOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut manifest = None;
        let mut output_dir = None;
        let mut json = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--manifest" => {
                    if manifest.is_some() {
                        return Err(
                            "usage error: equation-batch generate requires exactly one --manifest"
                                .to_string(),
                        );
                    }
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "usage error: --manifest requires a repository-relative TSV path"
                            .to_string()
                    })?;
                    if value.starts_with("--") {
                        return Err(
                            "usage error: --manifest requires a repository-relative TSV path"
                                .to_string(),
                        );
                    }
                    manifest = Some(PathBuf::from(value));
                }
                "--output-dir" => {
                    if output_dir.is_some() {
                        return Err(
                            "usage error: equation-batch generate requires exactly one --output-dir"
                                .to_string(),
                        );
                    }
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "usage error: --output-dir requires an output directory path".to_string()
                    })?;
                    if value.starts_with("--") {
                        return Err(
                            "usage error: --output-dir requires an output directory path"
                                .to_string(),
                        );
                    }
                    output_dir = Some(PathBuf::from(value));
                }
                "--json" => json = true,
                unknown if unknown.starts_with("--") => {
                    return Err(format!(
                        "usage error: unknown equation-batch generate flag `{unknown}`"
                    ));
                }
                unexpected => {
                    return Err(format!(
                        "usage error: unexpected equation-batch generate argument `{unexpected}`"
                    ));
                }
            }
            index += 1;
        }

        let manifest = manifest.ok_or_else(|| {
            "usage error: equation-batch generate requires exactly one --manifest".to_string()
        })?;
        let output_dir = output_dir.ok_or_else(|| {
            "usage error: equation-batch generate requires exactly one --output-dir".to_string()
        })?;

        Ok(Self {
            manifest,
            output_dir,
            json,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateResult {
    pub output_dir: PathBuf,
    pub source_manifest: String,
    pub source_manifest_hash: String,
    pub row_count: usize,
    pub packages: Vec<String>,
    pub generated_files: Vec<String>,
    pub summary_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PackageDependency {
    package: String,
    crate_names: Vec<String>,
    dependency_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FormulaSummary {
    formula_id: String,
    test_name: String,
    package: String,
    crate_name: String,
    runtime_symbol: String,
    output_variable: String,
    validation_status: String,
    test_strategy: String,
    line_number: usize,
    test_expression: String,
}

pub fn run_generate_command(root: &Path, options: &GenerateOptions) -> Result<(), String> {
    let result = generate_probe_crate(root, options)?;
    if options.json {
        print!("{}", result.summary_json);
    } else {
        print!("{}", render_human(&result));
    }
    Ok(())
}

pub fn generate_probe_crate(
    root: &Path,
    options: &GenerateOptions,
) -> Result<GenerateResult, String> {
    let manifest_path = normalize_manifest_argument(root, &options.manifest)?;
    let source_manifest = path_string(&manifest_path);
    let manifest_absolute = root.join(&manifest_path);
    let manifest_text = fs::read_to_string(&manifest_absolute)
        .map_err(|error| format!("{}: {error}", source_manifest))?;
    let source_manifest_hash = sha256_hex(manifest_text.as_bytes());
    let manifest = parse_equation_batch_manifest(&manifest_path, &manifest_text)?;
    let dependencies = package_dependencies_for_manifest(root, &manifest)?;
    let output_dir = resolve_output_dir(root, &options.output_dir)?;
    prepare_output_directory(&output_dir)?;

    create_repo_link(root, &output_dir)?;
    fs::create_dir_all(output_dir.join("src"))
        .map_err(|error| format!("{}: {error}", output_dir.join("src").display()))?;
    fs::create_dir_all(output_dir.join("tests"))
        .map_err(|error| format!("{}: {error}", output_dir.join("tests").display()))?;

    let generated_files = vec![
        "Cargo.toml".to_string(),
        RUST_CHECK_FILE.to_string(),
        SUMMARY_FILE.to_string(),
        MARKER_FILE.to_string(),
    ];
    let formulas = formula_summaries(&manifest.rows);
    let cargo_toml = render_cargo_toml(&dependencies);
    let rust_checks = render_rust_checks(&formulas);
    let marker = render_marker();
    let summary_json = render_summary_json(
        &source_manifest,
        &source_manifest_hash,
        &manifest.batch_id,
        manifest.row_count,
        &dependencies,
        &generated_files,
        &formulas,
    );

    write_generated_file(&output_dir.join("Cargo.toml"), &cargo_toml)?;
    write_generated_file(&output_dir.join(RUST_CHECK_FILE), &rust_checks)?;
    write_generated_file(&output_dir.join(MARKER_FILE), &marker)?;
    write_generated_file(&output_dir.join(SUMMARY_FILE), &summary_json)?;

    Ok(GenerateResult {
        output_dir,
        source_manifest,
        source_manifest_hash,
        row_count: manifest.row_count,
        packages: dependencies
            .iter()
            .map(|dependency| dependency.package.clone())
            .collect(),
        generated_files,
        summary_json,
    })
}

fn normalize_manifest_argument(root: &Path, path: &Path) -> Result<PathBuf, String> {
    let path_text = path_string(path);
    if path_text.starts_with('\\') || has_windows_absolute_prefix(&path_text) {
        return Err(format!(
            "usage error: manifest path must be repository-relative or inside the repository root: {}",
            path.display()
        ));
    }

    let relative = if path.is_absolute() {
        path.strip_prefix(root)
            .map_err(|_| {
                format!(
                    "usage error: manifest path must be inside the repository root: {}",
                    path.display()
                )
            })?
            .to_path_buf()
    } else {
        path.to_path_buf()
    };

    if relative.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(format!(
            "usage error: manifest path must be repository-relative without parent traversal: {}",
            path.display()
        ));
    }
    if relative.extension().and_then(|value| value.to_str()) != Some("tsv") {
        return Err(format!(
            "usage error: manifest path must point to a TSV file: {}",
            path.display()
        ));
    }

    Ok(relative)
}

pub(crate) fn resolve_output_dir(root: &Path, output_dir: &Path) -> Result<PathBuf, String> {
    let absolute_output = if output_dir.is_absolute() {
        output_dir.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|error| format!("current directory could not be read: {error}"))?
            .join(output_dir)
    };
    let resolved_output = resolve_path_for_containment(&absolute_output)?;
    let resolved_root = fs::canonicalize(root)
        .map_err(|error| format!("repository root could not be resolved: {error}"))?;
    let resolved_root = normalize_components(&resolved_root);

    if resolved_output == resolved_root || resolved_output.starts_with(&resolved_root) {
        return Err(format!(
            "usage error: --output-dir must be outside the AeroCodex repository by default: {}",
            output_dir.display()
        ));
    }

    Ok(resolved_output)
}

fn resolve_path_for_containment(path: &Path) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|error| format!("current directory could not be read: {error}"))?
            .join(path)
    };
    let mut probe = absolute.clone();
    let mut tail = Vec::<OsString>::new();

    while !probe.exists() {
        let Some(name) = probe.file_name() else {
            return Err(format!(
                "could not resolve output directory ancestor: {}",
                absolute.display()
            ));
        };
        tail.push(name.to_os_string());
        if !probe.pop() {
            return Err(format!(
                "could not resolve output directory ancestor: {}",
                absolute.display()
            ));
        }
    }

    let mut resolved =
        fs::canonicalize(&probe).map_err(|error| format!("{}: {error}", probe.display()))?;
    for component in tail.iter().rev() {
        resolved.push(component);
    }
    Ok(normalize_components(&resolved))
}

fn normalize_components(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => out.push(prefix.as_os_str()),
            Component::RootDir => out.push(Path::new("/")),
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(value) => out.push(value),
        }
    }
    out
}

fn prepare_output_directory(output_dir: &Path) -> Result<(), String> {
    if output_dir.exists() {
        if !output_dir.is_dir() {
            return Err(format!(
                "usage error: --output-dir exists but is not a directory: {}",
                output_dir.display()
            ));
        }
        let mut entries = fs::read_dir(output_dir)
            .map_err(|error| format!("{}: {error}", output_dir.display()))?;
        if entries.next().is_some() {
            return Err(format!(
                "usage error: --output-dir must be empty or absent; refusing non-empty directory: {}",
                output_dir.display()
            ));
        }
    } else {
        fs::create_dir_all(output_dir)
            .map_err(|error| format!("{}: {error}", output_dir.display()))?;
    }
    Ok(())
}

fn create_repo_link(root: &Path, output_dir: &Path) -> Result<(), String> {
    let link = output_dir.join(REPO_LINK_DIR);
    if fs::symlink_metadata(&link).is_ok() {
        return Err(format!(
            "internal error: generated repository link already exists: {}",
            link.display()
        ));
    }
    create_directory_symlink(root, &link)
        .map_err(|error| format!("{} -> {}: {error}", link.display(), root.display()))
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

fn package_dependencies_for_manifest(
    root: &Path,
    manifest: &EquationBatchManifest,
) -> Result<Vec<PackageDependency>, String> {
    let mut referenced = BTreeMap::<String, BTreeSet<String>>::new();
    for row in &manifest.rows {
        referenced
            .entry(row.package.clone())
            .or_default()
            .insert(row.crate_name.clone());
    }

    let workspace_packages = workspace_package_member_paths(root)?;
    let mut dependencies = Vec::new();
    for (package, crate_names) in referenced {
        let member_path = workspace_packages.get(&package).ok_or_else(|| {
            format!("manifest references package not found in workspace: {package}")
        })?;
        let mut dependency_path = PathBuf::from(REPO_LINK_DIR);
        dependency_path.push(member_path);
        dependencies.push(PackageDependency {
            package,
            crate_names: crate_names.into_iter().collect(),
            dependency_path: path_string(&dependency_path),
        });
    }
    Ok(dependencies)
}

fn workspace_package_member_paths(root: &Path) -> Result<BTreeMap<String, PathBuf>, String> {
    let workspace_manifest = root.join("Cargo.toml");
    let workspace_text = fs::read_to_string(&workspace_manifest)
        .map_err(|error| format!("{}: {error}", workspace_manifest.display()))?;
    let members = plan::parse_workspace_members(&workspace_text)?;
    let mut packages = BTreeMap::new();

    for member in members {
        let member_path = PathBuf::from(&member);
        if member_path.is_absolute()
            || member_path.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
            || has_windows_absolute_prefix(&member)
        {
            return Err(format!(
                "workspace member path is not repository-relative: {member}"
            ));
        }
        let member_manifest = root.join(&member_path).join("Cargo.toml");
        let member_text = fs::read_to_string(&member_manifest)
            .map_err(|error| format!("{}: {error}", member_manifest.display()))?;
        let package_name =
            plan::parse_toml_string_assignment_in_section(&member_text, "package", "name")
                .ok_or_else(|| format!("workspace member has no [package] name: {member}"))?;
        packages.insert(package_name, member_path);
    }

    Ok(packages)
}

fn formula_summaries(rows: &[EquationBatchRow]) -> Vec<FormulaSummary> {
    let mut used_names = BTreeMap::<String, usize>::new();
    rows.iter()
        .map(|row| {
            let base_name = sanitize_rust_identifier(&row.formula_id);
            let count = used_names.entry(base_name.clone()).or_insert(0);
            let test_name = if *count == 0 {
                base_name.clone()
            } else {
                format!("{base_name}_line_{}", row.line_number)
            };
            *count += 1;
            FormulaSummary {
                formula_id: row.formula_id.clone(),
                test_name,
                package: row.package.clone(),
                crate_name: row.crate_name.clone(),
                runtime_symbol: row.runtime_symbol.clone(),
                output_variable: row.output_variable.clone(),
                validation_status: row.validation_status.clone(),
                test_strategy: row.test_strategy.clone(),
                line_number: row.line_number,
                test_expression: row.test_expression.clone(),
            }
        })
        .collect()
}

fn sanitize_rust_identifier(value: &str) -> String {
    let mut out = String::new();
    let mut last_was_underscore = false;

    for character in value.chars() {
        let mapped = if character.is_ascii_alphanumeric() || character == '_' {
            Some(character.to_ascii_lowercase())
        } else {
            Some('_')
        };
        if let Some(character) = mapped {
            if character == '_' {
                if !last_was_underscore && !out.is_empty() {
                    out.push('_');
                    last_was_underscore = true;
                }
            } else {
                out.push(character);
                last_was_underscore = false;
            }
        }
    }

    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() {
        out.push_str("formula");
    }
    let starts_with_digit = out
        .as_bytes()
        .first()
        .map(|byte| byte.is_ascii_digit())
        .unwrap_or(false);
    if starts_with_digit {
        out.insert_str(0, "formula_");
    }
    out
}

fn render_cargo_toml(dependencies: &[PackageDependency]) -> String {
    let mut out = String::new();
    out.push_str("[package]\n");
    out.push_str("name = \"aerocodex-equation-batch-probe\"\n");
    out.push_str("version = \"0.0.0\"\n");
    out.push_str("edition = \"2021\"\n");
    out.push_str("publish = false\n\n");
    out.push_str("[dependencies]\n");
    for dependency in dependencies {
        writeln!(
            &mut out,
            "{} = {{ path = \"{}\" }}",
            dependency.package, dependency.dependency_path
        )
        .expect("writing to String cannot fail");
    }
    out
}

fn render_rust_checks(formulas: &[FormulaSummary]) -> String {
    let mut out = String::new();
    out.push_str("#![forbid(unsafe_code)]\n\n");
    out.push_str("#[cfg(test)]\n");
    out.push_str("mod generated_equation_batch {\n");
    for formula in formulas {
        writeln!(&mut out, "    // formula_id: {}", formula.formula_id)
            .expect("writing to String cannot fail");
        writeln!(&mut out, "    // manifest_line: {}", formula.line_number)
            .expect("writing to String cannot fail");
        out.push_str("    #[test]\n");
        writeln!(&mut out, "    fn {}() {{", formula.test_name)
            .expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "        assert!({}, \"formula_id {} test_expression returned false\");",
            formula.test_expression,
            escape_rust_string(&formula.formula_id)
        )
        .expect("writing to String cannot fail");
        out.push_str("    }\n\n");
    }
    out.push_str("}\n");
    out
}

fn render_marker() -> String {
    format!(
        "aerocodex_equation_batch_probe=generated\nschema_version={PROBE_SCHEMA_VERSION}\ngenerated_by={GENERATED_BY}\n"
    )
}

fn render_summary_json(
    source_manifest: &str,
    source_manifest_hash: &str,
    batch_id: &str,
    row_count: usize,
    dependencies: &[PackageDependency],
    generated_files: &[String],
    formulas: &[FormulaSummary],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    write_json_field(&mut out, 1, "schema_version", PROBE_SCHEMA_VERSION, true);
    write_json_field(&mut out, 1, "generated_by", GENERATED_BY, true);
    write_json_field(&mut out, 1, "source_manifest", source_manifest, true);
    write_json_field(
        &mut out,
        1,
        "source_manifest_hash",
        source_manifest_hash,
        true,
    );
    write_json_field(&mut out, 1, "batch_id", batch_id, true);
    writeln!(&mut out, "  \"row_count\": {row_count},").expect("writing to String cannot fail");

    out.push_str("  \"packages\": [\n");
    for (index, dependency) in dependencies.iter().enumerate() {
        out.push_str("    {\n");
        write_json_field(&mut out, 3, "package", &dependency.package, true);
        write_json_string_array_field(&mut out, 3, "crate_names", &dependency.crate_names, true);
        write_json_field(
            &mut out,
            3,
            "dependency_path",
            &dependency.dependency_path,
            false,
        );
        out.push_str("    }");
        if index + 1 != dependencies.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n");

    write_json_string_array_field(&mut out, 1, "generated_files", generated_files, true);

    out.push_str("  \"formulas\": [\n");
    for (index, formula) in formulas.iter().enumerate() {
        out.push_str("    {\n");
        write_json_field(&mut out, 3, "formula_id", &formula.formula_id, true);
        write_json_field(&mut out, 3, "test_name", &formula.test_name, true);
        write_json_field(&mut out, 3, "package", &formula.package, true);
        write_json_field(&mut out, 3, "crate_name", &formula.crate_name, true);
        write_json_field(&mut out, 3, "runtime_symbol", &formula.runtime_symbol, true);
        write_json_field(
            &mut out,
            3,
            "output_variable",
            &formula.output_variable,
            true,
        );
        write_json_field(
            &mut out,
            3,
            "validation_status",
            &formula.validation_status,
            true,
        );
        write_json_field(&mut out, 3, "test_strategy", &formula.test_strategy, true);
        writeln!(&mut out, "      \"line_number\": {},", formula.line_number)
            .expect("writing to String cannot fail");
        write_json_field(
            &mut out,
            3,
            "test_expression",
            &formula.test_expression,
            false,
        );
        out.push_str("    }");
        if index + 1 != formulas.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n");

    write_json_field(&mut out, 1, "safety_notice", SAFETY_NOTICE, true);
    write_json_string_array_field(
        &mut out,
        1,
        "non_claims",
        &NON_CLAIMS
            .iter()
            .map(|value| (*value).to_string())
            .collect::<Vec<_>>(),
        false,
    );
    out.push_str("}\n");
    out
}

fn write_json_field(out: &mut String, indent: usize, key: &str, value: &str, trailing: bool) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
    write_json_string(out, value);
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_json_string_array_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &[String],
    trailing: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": [");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        write_json_string(out, value);
    }
    out.push(']');
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
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
                write!(out, "\\u{:04x}", character as u32).expect("writing to String cannot fail");
            }
            character => out.push(character),
        }
    }
    out.push('"');
}

fn write_generated_file(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|error| format!("{}: {error}", path.display()))
}

fn render_human(result: &GenerateResult) -> String {
    format!(
        "equation_batch_probe_generation=PASS\nsource_manifest={}\noutput_dir={}\nrow_count={}\npackages={}\ngenerated_files={}\nsafety_notice={}\n",
        result.source_manifest,
        result.output_dir.display(),
        result.row_count,
        result.packages.join(","),
        result.generated_files.join(","),
        SAFETY_NOTICE
    )
}

fn escape_rust_string(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| character.escape_default())
        .collect()
}

fn path_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn has_windows_absolute_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = sha256_digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        write!(&mut out, "{byte:02x}").expect("writing to String cannot fail");
    }
    out
}

fn sha256_digest(bytes: &[u8]) -> [u8; 32] {
    const INITIAL_HASH: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut hash = INITIAL_HASH;
    let bit_len = (bytes.len() as u64) * 8;
    let mut padded = bytes.to_vec();
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in words.iter_mut().take(16).enumerate() {
            let offset = index * 4;
            *word = u32::from_be_bytes([
                chunk[offset],
                chunk[offset + 1],
                chunk[offset + 2],
                chunk[offset + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }

        let mut a = hash[0];
        let mut b = hash[1];
        let mut c = hash[2];
        let mut d = hash[3];
        let mut e = hash[4];
        let mut f = hash[5];
        let mut g = hash[6];
        let mut h = hash[7];

        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        hash[0] = hash[0].wrapping_add(a);
        hash[1] = hash[1].wrapping_add(b);
        hash[2] = hash[2].wrapping_add(c);
        hash[3] = hash[3].wrapping_add(d);
        hash[4] = hash[4].wrapping_add(e);
        hash[5] = hash[5].wrapping_add(f);
        hash[6] = hash[6].wrapping_add(g);
        hash[7] = hash[7].wrapping_add(h);
    }

    let mut digest = [0u8; 32];
    for (index, word) in hash.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::{
        generate_probe_crate, sanitize_rust_identifier, sha256_hex, GenerateOptions, MARKER_FILE,
    };

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn parse_args_requires_exactly_one_manifest_and_output_dir() {
        let options = GenerateOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/acx-m00-probe",
            "--json",
        ])
        .expect("valid generate args parse");

        assert_eq!(
            options.manifest.to_string_lossy(),
            "equation-batches/m00-canonical-units.tsv"
        );
        assert_eq!(options.output_dir.to_string_lossy(), "/tmp/acx-m00-probe");
        assert!(options.json);
    }

    #[test]
    fn parse_args_rejects_missing_manifest_missing_output_dir_and_unknown_flags() {
        let missing_manifest = GenerateOptions::parse_args(&["--output-dir", "/tmp/probe"])
            .expect_err("missing manifest must fail");
        assert!(missing_manifest.contains("usage error"));
        assert!(missing_manifest.contains("--manifest"));

        let missing_output = GenerateOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
        ])
        .expect_err("missing output dir must fail");
        assert!(missing_output.contains("usage error"));
        assert!(missing_output.contains("--output-dir"));

        let unknown = GenerateOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/probe",
            "--unknown-flag",
        ])
        .expect_err("unknown flags must fail");
        assert!(unknown.contains("usage error"));
        assert!(unknown.contains("unknown"));
    }

    #[test]
    fn sanitize_rust_identifier_preserves_formula_id_deterministically() {
        assert_eq!(
            sanitize_rust_identifier("formula_vault.m00.canonical.time-unit/µ"),
            "formula_vault_m00_canonical_time_unit"
        );
        assert_eq!(
            sanitize_rust_identifier("123.leading"),
            "formula_123_leading"
        );
    }

    #[test]
    fn sha256_hex_matches_empty_and_abc_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn generate_probe_crate_rejects_output_inside_repo() {
        let root = fake_repo_root("inside_repo_rejected");
        let manifest = root.join("equation-batches/m00.tsv");
        write_fake_repo(&root, &manifest);
        let options = GenerateOptions {
            manifest: PathBuf::from("equation-batches/m00.tsv"),
            output_dir: root.join("target/probe"),
            json: false,
        };

        let error = generate_probe_crate(&root, &options).expect_err("inside repo output fails");
        assert!(error.contains("usage error"));
        assert!(error.contains("outside the AeroCodex repository"));
        remove_dir_if_exists(&root);
    }

    #[test]
    fn generate_probe_crate_writes_expected_files_and_summary() {
        let root = fake_repo_root("writes_expected_files");
        let manifest = root.join("equation-batches/m00.tsv");
        write_fake_repo(&root, &manifest);
        let output_dir = fake_output_dir("writes_expected_files_probe");
        remove_dir_if_exists(&output_dir);
        let options = GenerateOptions {
            manifest: PathBuf::from("equation-batches/m00.tsv"),
            output_dir: output_dir.clone(),
            json: false,
        };

        let result = generate_probe_crate(&root, &options).expect("probe generation succeeds");

        assert_eq!(result.source_manifest, "equation-batches/m00.tsv");
        assert_eq!(result.row_count, 1);
        assert_eq!(result.packages, vec!["aero-codex-astrodynamics"]);
        assert!(output_dir.join("Cargo.toml").is_file());
        assert!(output_dir.join("src/lib.rs").is_file());
        assert!(output_dir.join("manifest_summary.json").is_file());
        assert!(output_dir.join(MARKER_FILE).is_file());
        let cargo_toml = fs::read_to_string(output_dir.join("Cargo.toml")).expect("Cargo.toml");
        assert!(cargo_toml.contains("path = \".aerocodex-repo/crates/aero-codex-astrodynamics\""));
        assert!(!cargo_toml.contains("git ="));
        assert!(!cargo_toml.contains("registry ="));
        let rust_checks = fs::read_to_string(output_dir.join("src/lib.rs")).expect("src/lib.rs");
        assert!(rust_checks.contains("#[test]"));
        assert!(
            rust_checks.contains("formula_id: formula_vault.m00.canonical.distance_to_canonical")
        );
        assert!(result.summary_json.contains("\"schema_version\""));
        assert!(result.summary_json.contains("\"source_manifest_hash\""));
        assert!(result.summary_json.contains("\"non_claims\""));

        remove_dir_if_exists(&output_dir);
        remove_dir_if_exists(&root);
    }

    fn fake_repo_root(label: &str) -> PathBuf {
        let path = unique_temp_path(label);
        remove_dir_if_exists(&path);
        fs::create_dir_all(&path).expect("create fake repo root");
        path
    }

    fn fake_output_dir(label: &str) -> PathBuf {
        unique_temp_path(label)
    }

    fn unique_temp_path(label: &str) -> PathBuf {
        let count = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        std::env::temp_dir().join(format!(
            "aerocodex_generate_test_{}_{}_{}",
            std::process::id(),
            count,
            label
        ))
    }

    fn write_fake_repo(root: &Path, manifest: &Path) {
        fs::create_dir_all(root.join("crates/aero-codex-astrodynamics/src"))
            .expect("create fake crate src");
        fs::create_dir_all(manifest.parent().expect("manifest parent")).expect("manifest dir");
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\n    \"crates/aero-codex-astrodynamics\",\n]\n",
        )
        .expect("write workspace Cargo.toml");
        fs::write(
            root.join("crates/aero-codex-astrodynamics/Cargo.toml"),
            "[package]\nname = \"aero-codex-astrodynamics\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[lib]\nname = \"aero_codex_astrodynamics\"\npath = \"src/lib.rs\"\n",
        )
        .expect("write crate Cargo.toml");
        fs::write(root.join("crates/aero-codex-astrodynamics/src/lib.rs"), "")
            .expect("write lib.rs");
        fs::write(
            manifest,
            "schema_version\tbatch_id\tformula_id\tpackage\tcrate_name\truntime_symbol\toutput_variable\tcontract_path\tvalidation_card_path\tsource_seed_path\tvalidation_status\ttest_strategy\ttest_expression\n\
aerocodex.equation_batch.v1\tm00\tformula_vault.m00.canonical.distance_to_canonical\taero-codex-astrodynamics\taero_codex_astrodynamics\tm00_distance_to_canonical\tcanonical_distance\tformula-vault/contracts/m00.yaml\tvalidation/cards/m00.yaml\tvalidation/source_registry/m00.yaml\tresearch_required\texact\ttrue\n",
        )
        .expect("write manifest");
    }

    fn remove_dir_if_exists(path: &Path) {
        if path.exists() {
            fs::remove_dir_all(path).expect("remove temp dir");
        }
    }
}
