use std::{env, path::PathBuf, process::Command};

const RELEASE_MANIFEST_SHA256: &str =
    "3a222b6a86c24f0eba8a1771308067c0c3e19235437e1070c8aeb8629a96db59";

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let repository_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("CLI crate is nested beneath the repository root");
    let manifest_hash_path = repository_root.join("release/release-manifest.sha256");

    println!("cargo:rerun-if-changed={}", manifest_hash_path.display());
    println!("cargo:rerun-if-env-changed=AEROCODEX_BUILD_COMMIT");
    println!("cargo:rerun-if-env-changed=AEROCODEX_RELEASE_MANIFEST_SHA256");

    let commit = env::var("AEROCODEX_BUILD_COMMIT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(repository_root)
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
        .unwrap_or_else(|| "unknown".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let manifest_hash = env::var("AEROCODEX_RELEASE_MANIFEST_SHA256")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            std::fs::read_to_string(&manifest_hash_path)
                .ok()
                .and_then(|sidecar| sidecar.split_whitespace().next().map(str::to_string))
        })
        .unwrap_or_else(|| RELEASE_MANIFEST_SHA256.to_string());

    assert!(
        manifest_hash.len() == 64
            && manifest_hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "release manifest SHA-256 input must be 64 lowercase hexadecimal characters"
    );
    assert_eq!(
        manifest_hash, RELEASE_MANIFEST_SHA256,
        "release manifest SHA-256 input differs from the reviewed release identity"
    );

    println!("cargo:rustc-env=AEROCODEX_BUILD_COMMIT={commit}");
    println!("cargo:rustc-env=AEROCODEX_BUILD_TARGET={target}");
    println!("cargo:rustc-env=AEROCODEX_RELEASE_MANIFEST_SHA256={manifest_hash}");
}
