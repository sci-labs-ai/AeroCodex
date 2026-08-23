$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

$script:TotalSteps = 16
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

Write-FriendTestInfo "AeroCodex local friend-test package"
Write-FriendTestInfo "repository root: $RepoRoot"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-FriendTestInfo "ERROR: cargo was not found on the command search path"
    Write-FriendTestInfo "Install Rust with cargo, rustfmt, and clippy before running the friend-test package."
    exit 127
}

if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Write-FriendTestInfo "rustc: $(& rustc --version)"
} else {
    Write-FriendTestInfo "rustc: not found on the command search path"
}
Write-FriendTestInfo "cargo: $(& cargo --version)"

$script:InGitCheckout = $false
if (Get-Command git -ErrorAction SilentlyContinue) {
    & git rev-parse --is-inside-work-tree *> $null
}
if ($LASTEXITCODE -eq 0 -and (Get-Command git -ErrorAction SilentlyContinue)) {
    $script:InGitCheckout = $true
    Write-FriendTestInfo "git commit: $(& git log -1 --format=%h)"
} else {
    Write-FriendTestInfo "source archive mode: Git metadata is not present"
}
$global:LASTEXITCODE = 0

$script:FriendBinary = $env:AEROCODEX_FRIEND_BINARY
$script:ExpectedCommit = $env:AEROCODEX_EXPECTED_COMMIT
if ($script:FriendBinary) {
    if (-not (Test-Path -PathType Leaf $script:FriendBinary)) {
        Write-FriendTestInfo "ERROR: AEROCODEX_FRIEND_BINARY does not exist: $($script:FriendBinary)"
        exit 126
    }
    Write-FriendTestInfo "downloaded binary: $($script:FriendBinary)"
}

function Test-SourceStatus {
    if ($script:InGitCheckout) {
        git status --short
    } else {
        if (-not (Test-Path Cargo.lock) -or -not (Test-Path release/release-manifest.toml)) {
            throw "source archive is missing the committed lockfile or release manifest"
        }
        Write-FriendTestInfo "source archive contains the committed lockfile and release manifest"
    }
}

function Test-SourceDiff {
    if ($script:InGitCheckout) {
        git diff --check
    } else {
        Write-FriendTestInfo "git diff is not applicable to a source archive; governed checksums run next"
    }
}

function Invoke-PublicCli {
    param([Parameter(ValueFromRemainingArguments = $true)][string[]]$CliArguments)
    if ($script:FriendBinary) {
        & $script:FriendBinary @CliArguments
    } else {
        cargo run --locked -p aero-codex-cli -- @CliArguments
    }
}

function Test-AllOrSourceArchive {
    if ($script:InGitCheckout) {
        cargo run --locked -p xtask -- verify --all
    } else {
        cargo run --locked -p xtask -- verify-source-archive
    }
}

function Test-ReleaseIdentityOrArchive {
    if ($script:InGitCheckout) {
        cargo run --locked -p xtask -- verify-release-identity
        return
    }
    if (-not $script:FriendBinary -or -not $script:ExpectedCommit) {
        throw "source archive mode requires AEROCODEX_FRIEND_BINARY and AEROCODEX_EXPECTED_COMMIT"
    }
    $version = (& $script:FriendBinary version --json | Out-String) | ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) { throw "downloaded binary version command failed" }
    $expectedManifest = ((Get-Content -Raw release/release-manifest.sha256) -split '\s+')[0]
    if ($version.git_commit -ne $script:ExpectedCommit) {
        throw "downloaded binary commit does not match the source archive"
    }
    if ($version.release_manifest_sha256 -ne $expectedManifest) {
        throw "downloaded binary manifest hash does not match the source archive"
    }
    Write-FriendTestInfo "downloaded binary commit and manifest hash match the source archive"
}

Invoke-FriendTestStep "git status --short" {
    Test-SourceStatus
}
Invoke-FriendTestStep "git diff --check" {
    Test-SourceDiff
}
Invoke-FriendTestStep "cargo run --locked -p xtask -- verify-checksums" {
    cargo run --locked -p xtask -- verify-checksums
}
Invoke-FriendTestStep "cargo fmt --all -- --check" {
    cargo fmt --all -- --check
}
Invoke-FriendTestStep "cargo check --locked --workspace --all-targets --all-features" {
    cargo check --locked --workspace --all-targets --all-features
}
Invoke-FriendTestStep "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings" {
    cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
}
Invoke-FriendTestStep "cargo test --locked --workspace --all-targets --all-features" {
    cargo test --locked --workspace --all-targets --all-features
}
Invoke-FriendTestStep "cargo run --locked -p aero-codex-cli -- version --json" {
    Invoke-PublicCli version --json
}
Invoke-FriendTestStep "cargo run --locked -p aero-codex-cli -- formula status-report --json" {
    Invoke-PublicCli formula status-report --json
}
Invoke-FriendTestStep "cargo run --locked -p aero-codex-cli -- self-check --json" {
    Invoke-PublicCli self-check --json
}
Invoke-FriendTestStep "cargo run --locked -p xtask -- verify --all" {
    Test-AllOrSourceArchive
}
Invoke-FriendTestStep "cargo run --locked -p xtask -- verify-release-manifest" {
    cargo run --locked -p xtask -- verify-release-manifest
}
Invoke-FriendTestStep "cargo run --locked -p xtask -- verify-release-identity" {
    Test-ReleaseIdentityOrArchive
}
Invoke-FriendTestStep "cargo run --locked -p xtask -- verify-generated" {
    cargo run --locked -p xtask -- verify-generated
}
Invoke-FriendTestStep "cargo run --locked -p xtask -- dependency-policy" {
    cargo run --locked -p xtask -- dependency-policy
}
Invoke-FriendTestStep 'RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps' {
    $PreviousRustdocFlags = $env:RUSTDOCFLAGS
    $env:RUSTDOCFLAGS = "-D warnings"
    try {
        cargo doc --locked --workspace --all-features --no-deps
    } finally {
        $env:RUSTDOCFLAGS = $PreviousRustdocFlags
    }
}

Write-FriendTestInfo "committed Cargo.lock was used with --locked"

Write-FriendTestInfo "completed all requested local checks"
Write-FriendTestInfo "Reminder: passing local checks does not prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval."
