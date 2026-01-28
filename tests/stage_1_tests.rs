use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn get_compiler_path() -> PathBuf {
    // Get the path to the compiled binary
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("rcc");
    path
}

fn get_all_stage_test_files(stage: &str) -> Vec<PathBuf> {
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("write_a_c_compiler");
    test_dir.push(stage);

    let mut files = Vec::new();

    // Read both valid and invalid subdirectories
    for subdir in &["valid", "invalid"] {
        let mut dir = test_dir.clone();
        dir.push(subdir);

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "c") {
                    files.push(path);
                }
            }
        }
    }

    files.sort();
    files
}

fn run_compiler(file_path: &PathBuf) -> std::process::Output {
    let compiler = get_compiler_path();

    Command::new(&compiler)
        .arg(file_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect(&format!("Failed to execute compiler on {:?}", file_path))
}

#[test]
fn stage_1_all_programs_should_lex() {
    let test_files = get_all_stage_test_files("stage_1");

    assert!(!test_files.is_empty(), "No test files found!");

    let mut failed_tests = Vec::new();

    for file in &test_files {
        let output = run_compiler(file);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            failed_tests.push((file.clone(), stderr.to_string()));
        }
    }

    if !failed_tests.is_empty() {
        eprintln!("\n{} test(s) failed:", failed_tests.len());
        for (file, stderr) in &failed_tests {
            eprintln!("  ❌ {}", file.file_name().unwrap().to_string_lossy());
            eprintln!(
                "     Error: {}",
                stderr.lines().next().unwrap_or("Unknown error").trim()
            );
        }
        panic!("{} test(s) failed", failed_tests.len());
    }

    println!("✅ All {} stage_1 tests passed!", test_files.len());
}
