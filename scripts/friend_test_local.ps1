$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

$script:TotalSteps = 15
$script:CurrentStep = 0

function Write-FriendTestInfo {
    param([Parameter(Mandatory = $true)][string]$Message)
    Write-Host "[friend-test] $Message"
}

function Invoke-FriendTestStep {
    param(
        [Parameter(Mandatory = $true)][string]$Label,
        [Parameter(Mandatory = $true)][scriptblock]$Command
    )

    $script:CurrentStep += 1
    Write-FriendTestInfo "step $($script:CurrentStep)/$($script:TotalSteps): $Label"
    & $Command
    if ($LASTEXITCODE -ne $null -and $LASTEXITCODE -ne 0) {
        throw "Friend-test step failed with exit code ${LASTEXITCODE}: $Label"
    }
    $global:LASTEXITCODE = 0
}

function Invoke-Sha256ManifestCheck {
    $Sha256Sum = Get-Command sha256sum -ErrorAction SilentlyContinue
    if ($Sha256Sum) {
        & $Sha256Sum.Source -c checksums/SHA256SUMS
        return
    }

    Write-FriendTestInfo "sha256sum: using PowerShell Get-FileHash fallback"
    $ManifestPath = Join-Path $RepoRoot "checksums/SHA256SUMS"
    foreach ($Line in Get-Content $ManifestPath) {
        if ([string]::IsNullOrWhiteSpace($Line)) {
            continue
        }
        if ($Line -notmatch '^([a-fA-F0-9]{64})\s+\*?(.+)$') {
            throw "Malformed checksum manifest line: $Line"
        }
        $Expected = $Matches[1].ToLowerInvariant()
        $RelativePath = $Matches[2]
        $NativeRelativePath = $RelativePath -replace '/', [System.IO.Path]::DirectorySeparatorChar
        $FilePath = Join-Path $RepoRoot $NativeRelativePath
        if (-not (Test-Path $FilePath)) {
            throw "Checksum manifest path missing: $RelativePath"
        }
        $Actual = (Get-FileHash -Algorithm SHA256 -Path $FilePath).Hash.ToLowerInvariant()
        if ($Actual -ne $Expected) {
            throw "Checksum mismatch for ${RelativePath}: expected ${Expected}, actual ${Actual}"
        }
        Write-FriendTestInfo "${RelativePath}: OK"
    }
    $global:LASTEXITCODE = 0
}

Write-FriendTestInfo "AeroCodex local friend-test package"
Write-FriendTestInfo "repository root: $RepoRoot"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-FriendTestInfo "ERROR: cargo was not found on the command search path"
    Write-FriendTestInfo "Install Rust with cargo, rustfmt, and clippy before running the friend-test package."
    exit 127
}

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-FriendTestInfo "ERROR: git was not found on the command search path"
    exit 127
}

$PythonCommand = Get-Command python -ErrorAction SilentlyContinue
if (-not $PythonCommand) {
    $PythonCommand = Get-Command python3 -ErrorAction SilentlyContinue
    if ($PythonCommand) {
        Write-FriendTestInfo "python: using python3 fallback because bare python was not found"
    }
}
if (-not $PythonCommand) {
    Write-FriendTestInfo "ERROR: neither python nor python3 was found on the command search path"
    exit 127
}
$PythonExe = $PythonCommand.Source

if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Write-FriendTestInfo "rustc: $(& rustc --version)"
} else {
    Write-FriendTestInfo "rustc: not found on the command search path"
}
Write-FriendTestInfo "cargo: $(& cargo --version)"
Write-FriendTestInfo "python command: $PythonExe ($(& $PythonExe --version 2>&1))"

& git rev-parse --is-inside-work-tree *> $null
if ($LASTEXITCODE -eq 0) {
    Write-FriendTestInfo "git commit: $(& git log -1 --format=%h)"
}
$global:LASTEXITCODE = 0

Invoke-FriendTestStep "git status --short" {
    git status --short
}
Invoke-FriendTestStep "git diff --check" {
    git diff --check
}
Invoke-FriendTestStep "sha256sum -c checksums/SHA256SUMS" {
    Invoke-Sha256ManifestCheck
}
Invoke-FriendTestStep "cargo fmt --all -- --check" {
    cargo fmt --all -- --check
}
Invoke-FriendTestStep "cargo check --workspace --all-targets --all-features" {
    cargo check --workspace --all-targets --all-features
}
Invoke-FriendTestStep "cargo clippy --workspace --all-targets --all-features -- -D warnings" {
    cargo clippy --workspace --all-targets --all-features -- -D warnings
}
Invoke-FriendTestStep "cargo test --workspace --all-targets --all-features" {
    cargo test --workspace --all-targets --all-features
}
Invoke-FriendTestStep "cargo run -p xtask -- verify --all" {
    cargo run -p xtask -- verify --all
}
Invoke-FriendTestStep "cargo run -p xtask -- dependency-policy" {
    cargo run -p xtask -- dependency-policy
}
Invoke-FriendTestStep "$PythonExe scripts/verify_thinfilm_artifact.py" {
    & $PythonExe scripts/verify_thinfilm_artifact.py
}
Invoke-FriendTestStep "$PythonExe nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature" {
    & $PythonExe nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
}
Invoke-FriendTestStep "$PythonExe nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json" {
    & $PythonExe nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
}
Invoke-FriendTestStep "$PythonExe nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl" {
    & $PythonExe nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl
}
Invoke-FriendTestStep "git diff --exit-code nomenclature/generated/terminology/index.jsonl" {
    git diff --exit-code nomenclature/generated/terminology/index.jsonl
}
Invoke-FriendTestStep "RUSTDOCFLAGS=\"-D warnings\" cargo doc --workspace --all-features --no-deps" {
    $PreviousRustdocFlags = $env:RUSTDOCFLAGS
    $env:RUSTDOCFLAGS = "-D warnings"
    try {
        cargo doc --workspace --all-features --no-deps
    } finally {
        $env:RUSTDOCFLAGS = $PreviousRustdocFlags
    }
}

if (Test-Path (Join-Path $RepoRoot "Cargo.lock")) {
    Write-FriendTestInfo "NOTE: a root Cargo.lock exists after the run. Do not submit it unless project policy changes."
}

Write-FriendTestInfo "completed all requested local checks"
Write-FriendTestInfo "Reminder: passing local checks does not prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval."
