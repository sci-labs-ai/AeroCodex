use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use super::{REGISTRY_JSON_PATH, REGISTRY_SCHEMA_VERSION, REGISTRY_SHA256_PATH};

pub const GENERATED_RUST_PATH: &str = "generated/rust/formula_registry.rs";
const GENERATED_RUST_BY: &str =
    "cargo run -p xtask -- formula-registry generate-rust --out generated/rust/formula_registry.rs";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateRustOptions {
    pub out: PathBuf,
}

impl GenerateRustOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut out: Option<PathBuf> = None;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--out" => {
                    if out.is_some() {
                        return Err(
                            "usage error: formula-registry generate-rust requires exactly one --out"
                                .to_string(),
                        );
                    }
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "usage error: --out requires the approved generated Rust path".to_string()
                    })?;
                    if value.starts_with("--") {
                        return Err(
                            "usage error: --out requires the approved generated Rust path"
                                .to_string(),
                        );
                    }
                    out = Some(PathBuf::from(value));
                }
                unknown if unknown.starts_with("--") => {
                    return Err(format!(
                        "usage error: unknown formula-registry generate-rust flag `{unknown}`"
                    ));
                }
                unexpected => {
                    return Err(format!(
                        "usage error: unexpected formula-registry generate-rust argument `{unexpected}`"
                    ));
                }
            }
            index += 1;
        }

        let out = out.ok_or_else(|| {
            "usage error: formula-registry generate-rust requires exactly one --out".to_string()
        })?;

        Ok(Self { out })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RustRegistry {
    formula_count: usize,
    formulas: Vec<RustFormulaEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RustFormulaEntry {
    formula_id: String,
    legacy_formula_id: Option<String>,
    aliases: Vec<String>,
    name: String,
    family: String,
    batch_id: Option<String>,
    status: String,
    quarantine_state: String,
    execution_policy: String,
    runtime_symbol: Option<String>,
    output_variable: Option<String>,
    input_names: Vec<String>,
    output_names: Vec<String>,
    implementation_package: Option<String>,
    implementation_crate: Option<String>,
    implementation_path: Option<String>,
    source_manifest_path: Option<String>,
    source_manifest_line: Option<String>,
    source_formula_id: Option<String>,
    sidecar_path: Option<String>,
    contract_path: Option<String>,
    validation_card_path: Option<String>,
    source_seed_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum JsonValue {
    Null,
    Bool,
    Number(String),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

pub fn run_generate_rust_command(root: &Path, options: &GenerateRustOptions) -> Result<(), String> {
    require_approved_output_path(&options.out)?;
    let registry = load_checked_formula_registry(root)?;
    let generated = render_rust_registry(&registry);
    let out_path = root.join(&options.out);

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    fs::write(&out_path, generated).map_err(|error| format!("{}: {error}", out_path.display()))?;
    println!(
        "wrote_formula_registry_rust={} formula_count={} source_json={} source_sha256={}",
        out_path.display(),
        registry.formula_count,
        REGISTRY_JSON_PATH,
        REGISTRY_SHA256_PATH
    );
    Ok(())
}

pub(crate) fn render_registry_module_from_json_text(json: &str) -> Result<String, String> {
    let registry = parse_registry_json(json)?;
    Ok(render_rust_registry(&registry))
}

pub(crate) fn parse_json_value(json: &str) -> Result<JsonValue, String> {
    JsonParser::new(json).parse()
}

fn require_approved_output_path(out: &Path) -> Result<(), String> {
    if out.is_absolute() || path_string(out) != GENERATED_RUST_PATH {
        return Err(format!(
            "formula-registry generate-rust must write the approved generated Rust path `{GENERATED_RUST_PATH}`"
        ));
    }
    Ok(())
}

fn load_checked_formula_registry(root: &Path) -> Result<RustRegistry, String> {
    let json_path = root.join(REGISTRY_JSON_PATH);
    let sha_path = root.join(REGISTRY_SHA256_PATH);
    let json_bytes = fs::read(&json_path)
        .map_err(|error| format!("cannot read {}: {error}", json_path.display()))?;
    let actual_digest = crate::equation_batch::generate::sha256_hex(&json_bytes);
    let expected_sha = format!("{actual_digest}  {REGISTRY_JSON_PATH}\n");
    let sidecar = fs::read_to_string(&sha_path)
        .map_err(|error| format!("cannot read {}: {error}", sha_path.display()))?;
    if sidecar != expected_sha {
        return Err(format!(
            "formula-registry generate-rust refused stale sha256 sidecar: expected `{}` in {}, found `{}`",
            expected_sha.trim_end(),
            REGISTRY_SHA256_PATH,
            sidecar.trim_end()
        ));
    }
    let json_text = String::from_utf8(json_bytes)
        .map_err(|error| format!("{} is not valid UTF-8: {error}", REGISTRY_JSON_PATH))?;
    parse_registry_json(&json_text)
}

fn parse_registry_json(json: &str) -> Result<RustRegistry, String> {
    let value = JsonParser::new(json).parse()?;
    let root = value.as_object("formula registry root")?;
    let schema_version = string_field(root, "schema_version", "formula registry root")?;
    if schema_version != REGISTRY_SCHEMA_VERSION {
        return Err(format!(
            "formula registry schema_version mismatch: expected `{REGISTRY_SCHEMA_VERSION}`, found `{schema_version}`"
        ));
    }
    let formula_count = usize_field(root, "formula_count", "formula registry root")?;
    let formulas_value = array_field(root, "formulas", "formula registry root")?;
    if formulas_value.len() != formula_count {
        return Err(format!(
            "formula registry formula_count mismatch: formula_count={formula_count} formulas_len={}",
            formulas_value.len()
        ));
    }

    let mut seen_formula_ids = BTreeSet::new();
    let mut seen_aliases = BTreeMap::<String, String>::new();
    let mut formulas = Vec::with_capacity(formulas_value.len());
    for (index, formula_value) in formulas_value.iter().enumerate() {
        let context = format!("formulas[{index}]");
        let formula = formula_from_json(formula_value, &context)?;
        if !seen_formula_ids.insert(formula.formula_id.clone()) {
            return Err(format!(
                "duplicate formula_id `{}` in checked registry JSON",
                formula.formula_id
            ));
        }
        for alias in &formula.aliases {
            if let Some(existing) = seen_aliases.insert(alias.clone(), formula.formula_id.clone()) {
                return Err(format!(
                    "duplicate alias `{alias}` maps to both `{existing}` and `{}` in checked registry JSON",
                    formula.formula_id
                ));
            }
        }
        formulas.push(formula);
    }

    formulas.sort_by(|left, right| left.formula_id.cmp(&right.formula_id));

    Ok(RustRegistry {
        formula_count,
        formulas,
    })
}

fn formula_from_json(value: &JsonValue, context: &str) -> Result<RustFormulaEntry, String> {
    let object = value.as_object(context)?;
    let formula_id = string_field(object, "formula_id", context)?;
    let legacy_formula_id = optional_string_field(object, "legacy_formula_id", context)?;
    let aliases = string_array_field(object, "aliases", context)?;
    let name = string_field(object, "name", context)?;
    let family = string_field(object, "family", context)?;
    let batch_id = optional_string_field(object, "batch_id", context)?;
    let status = string_field(object, "status", context)?;
    let quarantine_state = string_field(object, "quarantine_state", context)?;
    let execution_policy = string_field(object, "execution_policy", context)?;
    let runtime_symbol = optional_string_field(object, "runtime_symbol", context)?;
    let input_names = name_list_field(object, "inputs", context)?;
    let output_names = name_list_field(object, "outputs", context)?;
    let implementation_path = optional_object_field(object, "implementation_path", context)?;
    let source_trace = optional_object_field(object, "source_trace", context)?;

    Ok(RustFormulaEntry {
        formula_id,
        legacy_formula_id,
        aliases,
        name,
        family,
        batch_id,
        status,
        quarantine_state,
        execution_policy,
        runtime_symbol,
        output_variable: optional_string_from_object(implementation_path, "output_variable"),
        input_names,
        output_names,
        implementation_package: optional_string_from_object(implementation_path, "package"),
        implementation_crate: optional_string_from_object(implementation_path, "crate_name"),
        implementation_path: optional_string_from_object(implementation_path, "path"),
        source_manifest_path: optional_string_from_object(source_trace, "manifest_path"),
        source_manifest_line: optional_string_from_object(source_trace, "manifest_line"),
        source_formula_id: optional_string_from_object(source_trace, "source_formula_id"),
        sidecar_path: optional_string_from_object(source_trace, "sidecar_path"),
        contract_path: optional_string_from_object(source_trace, "contract_path"),
        validation_card_path: optional_string_from_object(source_trace, "validation_card_path"),
        source_seed_path: optional_string_from_object(source_trace, "source_seed_path"),
    })
}

fn render_rust_registry(registry: &RustRegistry) -> String {
    let mut out = String::new();
    out.push_str("// @generated by ");
    out.push_str(GENERATED_RUST_BY);
    out.push('\n');
    out.push_str("// Source: generated/formula_registry.json verified by generated/formula_registry.sha256.\n");
    out.push_str("// Checked-in deterministic registry metadata; do not edit by hand.\n\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    out.push_str("pub struct FormulaRegistryEntry {\n");
    out.push_str("    pub formula_id: &'static str,\n");
    out.push_str("    pub legacy_formula_id: Option<&'static str>,\n");
    out.push_str("    pub aliases: &'static [&'static str],\n");
    out.push_str("    pub name: &'static str,\n");
    out.push_str("    pub family: &'static str,\n");
    out.push_str("    pub batch_id: Option<&'static str>,\n");
    out.push_str("    pub status: &'static str,\n");
    out.push_str("    pub quarantine_state: &'static str,\n");
    out.push_str("    pub execution_policy: &'static str,\n");
    out.push_str("    pub runtime_symbol: Option<&'static str>,\n");
    out.push_str("    pub output_variable: Option<&'static str>,\n");
    out.push_str("    pub input_names: &'static [&'static str],\n");
    out.push_str("    pub output_names: &'static [&'static str],\n");
    out.push_str("    pub implementation_package: Option<&'static str>,\n");
    out.push_str("    pub implementation_crate: Option<&'static str>,\n");
    out.push_str("    pub implementation_path: Option<&'static str>,\n");
    out.push_str("    pub source_manifest_path: Option<&'static str>,\n");
    out.push_str("    pub source_manifest_line: Option<&'static str>,\n");
    out.push_str("    pub source_formula_id: Option<&'static str>,\n");
    out.push_str("    pub sidecar_path: Option<&'static str>,\n");
    out.push_str("    pub contract_path: Option<&'static str>,\n");
    out.push_str("    pub validation_card_path: Option<&'static str>,\n");
    out.push_str("    pub source_seed_path: Option<&'static str>,\n");
    out.push_str("}\n\n");
    writeln!(
        out,
        "pub const FORMULA_COUNT: usize = {};",
        registry.formula_count
    )
    .expect("write to string");
    out.push('\n');
    out.push_str("pub static FORMULA_REGISTRY: &[FormulaRegistryEntry] = &[\n");
    for formula in &registry.formulas {
        out.push_str("    FormulaRegistryEntry {\n");
        push_string_field(&mut out, "formula_id", &formula.formula_id);
        push_option_string_field(
            &mut out,
            "legacy_formula_id",
            formula.legacy_formula_id.as_deref(),
        );
        push_string_slice_field(&mut out, "aliases", &formula.aliases);
        push_string_field(&mut out, "name", &formula.name);
        push_string_field(&mut out, "family", &formula.family);
        push_option_string_field(&mut out, "batch_id", formula.batch_id.as_deref());
        push_string_field(&mut out, "status", &formula.status);
        push_string_field(&mut out, "quarantine_state", &formula.quarantine_state);
        push_string_field(&mut out, "execution_policy", &formula.execution_policy);
        push_option_string_field(
            &mut out,
            "runtime_symbol",
            formula.runtime_symbol.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "output_variable",
            formula.output_variable.as_deref(),
        );
        push_string_slice_field(&mut out, "input_names", &formula.input_names);
        push_string_slice_field(&mut out, "output_names", &formula.output_names);
        push_option_string_field(
            &mut out,
            "implementation_package",
            formula.implementation_package.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "implementation_crate",
            formula.implementation_crate.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "implementation_path",
            formula.implementation_path.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "source_manifest_path",
            formula.source_manifest_path.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "source_manifest_line",
            formula.source_manifest_line.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "source_formula_id",
            formula.source_formula_id.as_deref(),
        );
        push_option_string_field(&mut out, "sidecar_path", formula.sidecar_path.as_deref());
        push_option_string_field(&mut out, "contract_path", formula.contract_path.as_deref());
        push_option_string_field(
            &mut out,
            "validation_card_path",
            formula.validation_card_path.as_deref(),
        );
        push_option_string_field(
            &mut out,
            "source_seed_path",
            formula.source_seed_path.as_deref(),
        );
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");
    out.push_str(
        "pub fn find_by_formula_id(formula_id: &str) -> Option<&'static FormulaRegistryEntry> {\n",
    );
    out.push_str("    FORMULA_REGISTRY\n");
    out.push_str("        .iter()\n");
    out.push_str("        .find(|entry| entry.formula_id == formula_id)\n");
    out.push_str("}\n\n");
    out.push_str("pub fn find_by_alias(alias: &str) -> Option<&'static FormulaRegistryEntry> {\n");
    out.push_str("    FORMULA_REGISTRY\n");
    out.push_str("        .iter()\n");
    out.push_str(
        "        .find(|entry| entry.aliases.iter().any(|candidate| *candidate == alias))\n",
    );
    out.push_str("}\n");
    out
}

fn push_string_field(out: &mut String, name: &str, value: &str) {
    writeln!(out, "        {name}: {},", rust_string_literal(value)).expect("write to string");
}

fn push_option_string_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => writeln!(out, "        {name}: Some({}),", rust_string_literal(value))
            .expect("write to string"),
        None => writeln!(out, "        {name}: None,").expect("write to string"),
    }
}

fn push_string_slice_field(out: &mut String, name: &str, values: &[String]) {
    write!(out, "        {name}: &[").expect("write to string");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&rust_string_literal(value));
    }
    out.push_str("],\n");
}

fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

fn string_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<String, String> {
    match object.get(key) {
        Some(JsonValue::String(value)) => Ok(value.clone()),
        Some(_) => Err(format!("{context}.{key} must be a string")),
        None => Err(format!("{context}.{key} is missing")),
    }
}

fn optional_string_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<Option<String>, String> {
    match object.get(key) {
        Some(JsonValue::String(value)) => Ok(Some(value.clone())),
        Some(JsonValue::Null) => Ok(None),
        Some(_) => Err(format!("{context}.{key} must be a string or null")),
        None => Err(format!("{context}.{key} is missing")),
    }
}

fn usize_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<usize, String> {
    match object.get(key) {
        Some(JsonValue::Number(value)) => value
            .parse::<usize>()
            .map_err(|error| format!("{context}.{key} must be a non-negative integer: {error}")),
        Some(_) => Err(format!("{context}.{key} must be a number")),
        None => Err(format!("{context}.{key} is missing")),
    }
}

fn array_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<&'a [JsonValue], String> {
    match object.get(key) {
        Some(JsonValue::Array(values)) => Ok(values),
        Some(_) => Err(format!("{context}.{key} must be an array")),
        None => Err(format!("{context}.{key} is missing")),
    }
}

fn string_array_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<Vec<String>, String> {
    array_field(object, key, context)?
        .iter()
        .enumerate()
        .map(|(index, value)| match value {
            JsonValue::String(item) => Ok(item.clone()),
            _ => Err(format!("{context}.{key}[{index}] must be a string")),
        })
        .collect()
}

fn optional_object_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<Option<&'a BTreeMap<String, JsonValue>>, String> {
    match object.get(key) {
        Some(JsonValue::Object(value)) => Ok(Some(value)),
        Some(JsonValue::Null) => Ok(None),
        Some(_) => Err(format!("{context}.{key} must be an object or null")),
        None => Err(format!("{context}.{key} is missing")),
    }
}

fn optional_string_from_object(
    object: Option<&BTreeMap<String, JsonValue>>,
    key: &str,
) -> Option<String> {
    match object.and_then(|fields| fields.get(key)) {
        Some(JsonValue::String(value)) if !value.is_empty() => Some(value.clone()),
        _ => None,
    }
}

fn name_list_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<Vec<String>, String> {
    array_field(object, key, context)?
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let item_context = format!("{context}.{key}[{index}]");
            let item_object = item.as_object(&item_context)?;
            string_field(item_object, "name", &item_context)
        })
        .collect()
}

impl JsonValue {
    fn as_object(&self, context: &str) -> Result<&BTreeMap<String, JsonValue>, String> {
        match self {
            JsonValue::Object(object) => Ok(object),
            _ => Err(format!("{context} must be a JSON object")),
        }
    }
}

struct JsonParser<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, offset: 0 }
    }

    fn parse(mut self) -> Result<JsonValue, String> {
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.offset != self.input.len() {
            return Err(format!("unexpected trailing JSON at byte {}", self.offset));
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('n') => {
                self.consume_literal("null")?;
                Ok(JsonValue::Null)
            }
            Some('t') => {
                self.consume_literal("true")?;
                Ok(JsonValue::Bool)
            }
            Some('f') => {
                self.consume_literal("false")?;
                Ok(JsonValue::Bool)
            }
            Some('"') => self.parse_string().map(JsonValue::String),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some('-' | '0'..='9') => self.parse_number().map(JsonValue::Number),
            Some(character) => Err(format!(
                "unexpected JSON character `{character}` at byte {}",
                self.offset
            )),
            None => Err("unexpected end of JSON".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.expect_char('{')?;
        let mut object = BTreeMap::new();
        self.skip_whitespace();
        if self.consume_char('}') {
            return Ok(JsonValue::Object(object));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect_char(':')?;
            let value = self.parse_value()?;
            if object.insert(key.clone(), value).is_some() {
                return Err(format!("duplicate JSON object key `{key}`"));
            }
            self.skip_whitespace();
            if self.consume_char('}') {
                break;
            }
            self.expect_char(',')?;
        }
        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.expect_char('[')?;
        let mut values = Vec::new();
        self.skip_whitespace();
        if self.consume_char(']') {
            return Ok(JsonValue::Array(values));
        }
        loop {
            values.push(self.parse_value()?);
            self.skip_whitespace();
            if self.consume_char(']') {
                break;
            }
            self.expect_char(',')?;
        }
        Ok(JsonValue::Array(values))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect_char('"')?;
        let mut out = String::new();
        loop {
            let character = self.next_char().ok_or_else(|| {
                format!(
                    "unterminated JSON string starting before byte {}",
                    self.offset
                )
            })?;
            match character {
                '"' => return Ok(out),
                '\\' => out.push(self.parse_escape()?),
                character if character <= '\u{001F}' => {
                    return Err(format!(
                        "unescaped control character in JSON string at byte {}",
                        self.offset
                    ));
                }
                character => out.push(character),
            }
        }
    }

    fn parse_escape(&mut self) -> Result<char, String> {
        let escaped = self
            .next_char()
            .ok_or_else(|| "unterminated JSON escape".to_string())?;
        match escaped {
            '"' => Ok('"'),
            '\\' => Ok('\\'),
            '/' => Ok('/'),
            'b' => Ok('\u{0008}'),
            'f' => Ok('\u{000C}'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'u' => self.parse_unicode_escape(),
            _ => Err(format!("invalid JSON escape `\\{escaped}`")),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<char, String> {
        let mut value = 0u32;
        for _ in 0..4 {
            let character = self
                .next_char()
                .ok_or_else(|| "incomplete JSON unicode escape".to_string())?;
            let digit = character
                .to_digit(16)
                .ok_or_else(|| format!("invalid JSON unicode escape digit `{character}`"))?;
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| format!("invalid JSON unicode scalar U+{value:04X}"))
    }

    fn parse_number(&mut self) -> Result<String, String> {
        let start = self.offset;
        self.consume_char('-');
        match self.peek_char() {
            Some('0') => {
                self.next_char();
            }
            Some('1'..='9') => {
                self.next_char();
                while matches!(self.peek_char(), Some('0'..='9')) {
                    self.next_char();
                }
            }
            _ => return Err(format!("invalid JSON number at byte {start}")),
        }
        if self.consume_char('.') {
            if !matches!(self.peek_char(), Some('0'..='9')) {
                return Err(format!(
                    "invalid JSON number fraction at byte {}",
                    self.offset
                ));
            }
            while matches!(self.peek_char(), Some('0'..='9')) {
                self.next_char();
            }
        }
        if matches!(self.peek_char(), Some('e' | 'E')) {
            self.next_char();
            let _ = self.consume_char('+') || self.consume_char('-');
            if !matches!(self.peek_char(), Some('0'..='9')) {
                return Err(format!(
                    "invalid JSON number exponent at byte {}",
                    self.offset
                ));
            }
            while matches!(self.peek_char(), Some('0'..='9')) {
                self.next_char();
            }
        }
        Ok(self.input[start..self.offset].to_string())
    }

    fn consume_literal(&mut self, literal: &str) -> Result<(), String> {
        if self.input[self.offset..].starts_with(literal) {
            self.offset += literal.len();
            Ok(())
        } else {
            Err(format!(
                "expected JSON literal `{literal}` at byte {}",
                self.offset
            ))
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_char(), Some(' ' | '\n' | '\r' | '\t')) {
            self.next_char();
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), String> {
        match self.next_char() {
            Some(actual) if actual == expected => Ok(()),
            Some(actual) => Err(format!(
                "expected JSON character `{expected}`, found `{actual}` at byte {}",
                self.offset
            )),
            None => Err(format!(
                "expected JSON character `{expected}`, found end of input"
            )),
        }
    }

    fn consume_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.next_char();
            true
        } else {
            false
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.offset..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let character = self.peek_char()?;
        self.offset += character.len_utf8();
        Some(character)
    }
}

fn path_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn parse_args_requires_exactly_one_approved_out() {
        let options =
            GenerateRustOptions::parse_args(&["--out", "generated/rust/formula_registry.rs"])
                .expect("valid generate-rust args parse");
        assert_eq!(
            options.out.to_string_lossy(),
            "generated/rust/formula_registry.rs"
        );

        let missing_out = GenerateRustOptions::parse_args(&[]).expect_err("missing out fails");
        assert!(missing_out.contains("usage error"));
        assert!(missing_out.contains("--out"));

        let duplicate_out = GenerateRustOptions::parse_args(&[
            "--out",
            "generated/rust/formula_registry.rs",
            "--out",
            "generated/rust/formula_registry.rs",
        ])
        .expect_err("duplicate out fails");
        assert!(duplicate_out.contains("usage error"));
        assert!(duplicate_out.contains("--out"));

        let unknown = GenerateRustOptions::parse_args(&[
            "--out",
            "generated/rust/formula_registry.rs",
            "--check",
        ])
        .expect_err("check is not owned by RR-016");
        assert!(unknown.contains("usage error"));
        assert!(unknown.contains("unknown"));
    }

    #[test]
    fn rejects_unapproved_output_path() {
        let options = GenerateRustOptions {
            out: "generated/rust/other.rs".into(),
        };
        let error = run_generate_rust_command(Path::new("."), &options)
            .expect_err("unapproved output fails closed");
        assert!(error.contains("generated/rust/formula_registry.rs"));
    }

    #[test]
    fn generates_rust_registry_from_checked_json() {
        let root = fake_repo_root("generate_rust");
        write_checked_registry(&root, fixture_registry_json());

        let options = GenerateRustOptions {
            out: "generated/rust/formula_registry.rs".into(),
        };
        run_generate_rust_command(&root, &options).expect("generate-rust succeeds");

        let generated = fs::read_to_string(root.join("generated/rust/formula_registry.rs"))
            .expect("generated rust exists");
        assert!(generated.contains("pub const FORMULA_COUNT: usize = 2;"));
        assert!(generated.contains("pub static FORMULA_REGISTRY"));
        assert!(generated.contains("formula_id: \"m00.angle.deg_to_rad\""));
        assert!(generated.contains("legacy_formula_id: Some(\"formula_vault.m00.angle.deg2rad\")"));
        assert!(generated.contains("output_variable: Some(\"angle_radians\")"));
        assert!(generated.contains("input_names: &[\"degrees\"]"));
        assert!(generated.contains("output_names: &[\"angle_radians\"]"));
        assert!(generated.contains("pub fn find_by_formula_id"));
        assert!(generated.contains("pub fn find_by_alias"));
        assert!(!generated.to_lowercase().contains("generated_at"));
    }

    #[test]
    fn stale_registry_sha_fails_closed_before_generation() {
        let root = fake_repo_root("stale_sha");
        write_registry_with_sha(
            &root,
            fixture_registry_json(),
            "bad-digest  generated/formula_registry.json\n",
        );
        let options = GenerateRustOptions {
            out: "generated/rust/formula_registry.rs".into(),
        };
        let error = run_generate_rust_command(&root, &options).expect_err("stale sha fails");
        assert!(error.contains("sha256"));
        assert!(!root.join("generated/rust/formula_registry.rs").exists());
    }

    fn fake_repo_root(name: &str) -> PathBuf {
        let mut root = std::env::temp_dir();
        root.push(format!(
            "aerocodex_rr016_{name}_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after epoch")
                .as_nanos()
        ));
        fs::create_dir_all(root.join("generated")).expect("create generated dir");
        root
    }

    fn write_checked_registry(root: &Path, json: &str) {
        let digest = crate::equation_batch::generate::sha256_hex(json.as_bytes());
        write_registry_with_sha(
            root,
            json,
            &format!("{digest}  generated/formula_registry.json\n"),
        );
    }

    fn write_registry_with_sha(root: &Path, json: &str, sha_text: &str) {
        fs::create_dir_all(root.join("generated")).expect("create generated dir");
        fs::write(root.join("generated/formula_registry.json"), json).expect("write registry json");
        fs::write(root.join("generated/formula_registry.sha256"), sha_text)
            .expect("write registry sha");
    }

    fn fixture_registry_json() -> &'static str {
        r#"{
  "schema_version":"aerocodex.formula_registry.v1",
  "generator_version":"xtask-formula-registry-v1",
  "generated_by":"cargo run -p xtask -- formula-registry generate",
  "source_hash":"sha256:test",
  "formula_count":2,
  "non_claims":[],
  "formulas":[
    {
      "formula_id":"m00.angle.deg_to_rad",
      "legacy_formula_id":"formula_vault.m00.angle.deg2rad",
      "aliases":["formula_vault.m00.angle.deg2rad"],
      "name":"Degrees to radians",
      "summary":"Inventory metadata only.",
      "family":"m00.angle",
      "batch_id":"m00-angle-vector",
      "status":"research_required",
      "quarantine_state":"below_execution_threshold",
      "execution_policy":"blocked",
      "source_trace":{
        "manifest_path":"equation-batches/m00-angle-vector.tsv",
        "manifest_line":"2",
        "source_formula_id":"formula_vault.m00.angle.deg2rad",
        "sidecar_path":"formula-schemas/m00/angle/deg_to_rad.yaml",
        "contract_path":"formula-vault/contracts/m00_angle_unit_conversions_contract.yaml",
        "validation_card_path":"validation/cards/validation_formula_vault_m00_angle_unit_conversions.yaml",
        "source_seed_path":"validation/source_registry/source_formula_vault_m00_angle_unit_conversions.yaml"
      },
      "inputs":[{"name":"degrees","type":"f64","unit":"deg","required":true}],
      "outputs":[{"name":"angle_radians","type":"f64","unit":"rad"}],
      "units":{"input_unit":"deg","output_unit":"rad"},
      "domain_constraints":[],
      "implementation_path":{
        "package":"aero-codex-astrodynamics",
        "crate_name":"aero_codex_astrodynamics",
        "path":"crates/aero-codex-astrodynamics/src/lib.rs",
        "runtime_symbol":"m00_degrees_to_radians",
        "output_variable":"angle_radians"
      },
      "runtime_symbol":"m00_degrees_to_radians",
      "test_vectors":[],
      "warnings":[]
    },
    {
      "formula_id":"m00.canonical.time_unit_from_mu_du",
      "legacy_formula_id":"formula_vault.m00.canonical.time_unit_from_mu_du",
      "aliases":["formula_vault.m00.canonical.time_unit_from_mu_du"],
      "name":"Time Unit From Mu Du",
      "summary":"Inventory metadata only.",
      "family":"m00.canonical",
      "batch_id":"m00-canonical-units",
      "status":"research_required",
      "quarantine_state":"below_execution_threshold",
      "execution_policy":"blocked",
      "source_trace":{"manifest_path":"equation-batches/m00-canonical-units.tsv"},
      "inputs":[],
      "outputs":[{"name":"time_unit","type":"f64"}],
      "units":null,
      "domain_constraints":[],
      "implementation_path":{
        "package":"aero-codex-astrodynamics",
        "crate_name":"aero_codex_astrodynamics",
        "runtime_symbol":"m00_canonical_time_unit_from_mu_du",
        "output_variable":"time_unit"
      },
      "runtime_symbol":"m00_canonical_time_unit_from_mu_du",
      "test_vectors":[],
      "warnings":[]
    }
  ]
}
"#
    }
}
