use std::{
    env, fs,
    path::Path,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn command(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}
fn hash_files(root: &Path, hash: &mut u64) {
    let mut paths: Vec<_> = fs::read_dir(root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            hash_files(&path, hash);
        } else if path.extension().is_some_and(|e| e == "rs")
            || path
                .file_name()
                .is_some_and(|n| n == "Cargo.toml" || n == "Cargo.lock")
        {
            // Include relative traversal names and delimiters as well as contents.
            for byte in path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .bytes()
                .chain([0])
                .chain(fs::read(&path).unwrap())
                .chain([0])
            {
                *hash = (*hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
            }
        }
    }
}
fn emit(key: &str, value: impl std::fmt::Display) {
    println!("cargo:rustc-env={key}={value}");
}
fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&manifest);
    let lib = root.join("../../iOverlay").canonicalize().unwrap();
    for path in [
        root.join("src"),
        root.join("Cargo.toml"),
        root.join("Cargo.lock"),
        root.join("build.rs"),
        lib.join("src"),
        lib.join("Cargo.toml"),
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    for git_path in ["HEAD", "index", "packed-refs"] {
        let path = command(
            "git",
            &[
                "-C",
                lib.to_str().unwrap(),
                "rev-parse",
                "--path-format=absolute",
                "--git-path",
                git_path,
            ],
        );
        if path != "unknown" {
            println!("cargo:rerun-if-changed={path}");
        }
    }
    let branch_ref = command(
        "git",
        &["-C", lib.to_str().unwrap(), "symbolic-ref", "-q", "HEAD"],
    );
    if branch_ref != "unknown" {
        let path = command(
            "git",
            &[
                "-C",
                lib.to_str().unwrap(),
                "rev-parse",
                "--path-format=absolute",
                "--git-path",
                &branch_ref,
            ],
        );
        if path != "unknown" {
            println!("cargo:rerun-if-changed={path}");
        }
    }
    let text = fs::read_to_string(lib.join("Cargo.toml")).unwrap();
    let version = text
        .lines()
        .find_map(|s| s.strip_prefix("version = "))
        .unwrap()
        .trim_matches('"');
    emit("BENCH_LIBRARY_VERSION", version);
    let mut hash = 0xcbf29ce484222325;
    hash_files(&lib.join("src"), &mut hash);
    for byte in text.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
    }
    emit("BENCH_LIBRARY_SOURCE", format!("{hash:016x}"));
    let mut hash = 0xcbf29ce484222325;
    hash_files(&root.join("src"), &mut hash);
    for name in ["Cargo.toml", "Cargo.lock", "build.rs"] {
        for byte in fs::read(root.join(name)).unwrap() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
    }
    emit("BENCH_RUNNER_SOURCE", format!("{hash:016x}"));
    emit(
        "BENCH_RUSTC",
        command(&env::var("RUSTC").unwrap(), &["--version"]),
    );
    emit("BENCH_TARGET", env::var("TARGET").unwrap());
    emit("BENCH_PROFILE", env::var("PROFILE").unwrap());
    emit("BENCH_OPT_LEVEL", env::var("OPT_LEVEL").unwrap());
    emit("BENCH_DEBUG", env::var("DEBUG").unwrap());
    emit(
        "BENCH_RUSTFLAGS",
        env::var("CARGO_ENCODED_RUSTFLAGS")
            .unwrap_or_default()
            .replace('\x1f', " "),
    );
    emit(
        "BENCH_BUILD_UNIX",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    emit(
        "BENCH_LIBRARY_REV",
        command("git", &["-C", lib.to_str().unwrap(), "rev-parse", "HEAD"]),
    );
    let status = command(
        "git",
        &[
            "-C",
            lib.to_str().unwrap(),
            "status",
            "--porcelain",
            "--",
            ".",
        ],
    );
    emit(
        "BENCH_LIBRARY_DIRTY",
        if status == "unknown" {
            "unknown"
        } else if status.is_empty() {
            "false"
        } else {
            "true"
        },
    );
}
