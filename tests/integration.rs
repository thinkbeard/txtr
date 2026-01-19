use std::process::Command;

fn get_binary_path() -> String {
    // Build the binary first, then get its path
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to build binary");

    if !output.status.success() {
        panic!(
            "Failed to build: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    format!("{}/target/release/txtr", env!("CARGO_MANIFEST_DIR"))
}

fn run_txtr(args: &[&str]) -> std::process::Output {
    let binary = get_binary_path();
    Command::new(&binary)
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command")
}

// Test image path
const TEST_IMAGE: &str = "assets/castle.jpg";

#[test]
fn basic_conversion_works() {
    let output = run_txtr(&[TEST_IMAGE, "-w", "10"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty());
}

#[test]
fn empty_chars_rejected() {
    let output = run_txtr(&[TEST_IMAGE, "--chars", ""]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("2 or more characters"));
}

#[test]
fn single_char_rejected() {
    let output = run_txtr(&[TEST_IMAGE, "--chars", "x"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("2 or more characters"));
}

#[test]
fn two_chars_accepted() {
    let output = run_txtr(&[TEST_IMAGE, "--chars", "ab", "-w", "10"]);
    assert!(output.status.success());
}

#[test]
fn invalid_ramp_warns() {
    let output = run_txtr(&[TEST_IMAGE, "--ramp", "invalid_ramp", "-w", "10"]);
    // Should still succeed but with warning
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown ramp"));
    assert!(stderr.contains("standard"));
}

#[test]
fn invalid_encoder_warns() {
    let output = run_txtr(&[TEST_IMAGE, "--encoder", "invalid_encoder", "-w", "10"]);
    // Should still succeed but with warning
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown encoder"));
    assert!(stderr.contains("luma709"));
}

#[test]
fn blocks_with_dither_warns() {
    let output = run_txtr(&[TEST_IMAGE, "--blocks", "--dither", "-w", "10"]);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--blocks mode ignores"));
    assert!(stderr.contains("--dither"));
}

#[test]
fn blocks_with_chars_warns() {
    let output = run_txtr(&[TEST_IMAGE, "--blocks", "--chars", "abc", "-w", "10"]);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--blocks mode ignores"));
    assert!(stderr.contains("--chars"));
}

#[test]
fn nan_channel_rejected() {
    // NaN values from command line would typically come from invalid parsing
    // clap handles this, but let's test with infinity which parses as f64
    let output = run_txtr(&[TEST_IMAGE, "--red", "nan", "-w", "10"]);
    // clap should reject this as invalid f64
    assert!(!output.status.success());
}

#[test]
fn zero_width_rejected() {
    let output = run_txtr(&[TEST_IMAGE, "--width", "0"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("width"));
}

#[test]
fn channel_values_clamped() {
    // Values outside 0.0-1.0 should be clamped with warning
    let output = run_txtr(&[TEST_IMAGE, "--red", "2.0", "-w", "10"]);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("clamped"));
}

#[test]
fn negative_channel_rejected_by_clap() {
    // Negative values starting with '-' are parsed by clap as flags
    // This is expected CLI behavior, not a bug
    let output = run_txtr(&[TEST_IMAGE, "--green", "-1.0", "-w", "10"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument"));
}

#[test]
fn valid_encoders_work() {
    for encoder in &["red", "green", "blue", "alpha", "luma601", "luma709"] {
        let output = run_txtr(&[TEST_IMAGE, "--encoder", encoder, "-w", "10"]);
        assert!(
            output.status.success(),
            "Encoder {} should work",
            encoder
        );
    }
}

#[test]
fn valid_ramps_work() {
    for ramp in &["standard", "dense", "blocks", "simple"] {
        let output = run_txtr(&[TEST_IMAGE, "--ramp", ramp, "-w", "10"]);
        assert!(output.status.success(), "Ramp {} should work", ramp);
    }
}

#[test]
fn color_flag_works() {
    let output = run_txtr(&[TEST_IMAGE, "--color", "-w", "10"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should contain ANSI escape codes
    assert!(stdout.contains("\x1b["));
}

#[test]
fn blocks_mode_works() {
    let output = run_txtr(&[TEST_IMAGE, "--blocks", "-w", "10"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should contain half-block character
    assert!(stdout.contains('▀'));
}

#[test]
fn invert_flag_works() {
    let output = run_txtr(&[TEST_IMAGE, "--invert", "-w", "10"]);
    assert!(output.status.success());
}

#[test]
fn outline_flag_works() {
    let output = run_txtr(&[TEST_IMAGE, "--outline", "-w", "10"]);
    assert!(output.status.success());
}

#[test]
fn print_in_order_flag_works() {
    let output = run_txtr(&[TEST_IMAGE, "--print-in-order", "-w", "10"]);
    assert!(output.status.success());
}

#[test]
fn dither_flag_works() {
    let output = run_txtr(&[TEST_IMAGE, "--dither", "-w", "10"]);
    assert!(output.status.success());
}

#[test]
fn nonexistent_file_error() {
    let output = run_txtr(&["nonexistent_file.jpg"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error"));
}

#[test]
fn fontsize_works() {
    let output = run_txtr(&[TEST_IMAGE, "--fontsize", "0.5", "-w", "10"]);
    assert!(output.status.success());
}

#[test]
fn tiny_fontsize_rejected() {
    let output = run_txtr(&[TEST_IMAGE, "--fontsize", "0.001", "-w", "10"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("fontsize"));
}

#[test]
fn level_threshold_works() {
    let output = run_txtr(&[TEST_IMAGE, "--print-in-order", "--level", "200", "-w", "10"]);
    assert!(output.status.success());
}

// Case sensitivity tests (documenting current behavior)
#[test]
fn encoder_is_case_sensitive() {
    // Uppercase encoder name should trigger fallback warning
    let output = run_txtr(&[TEST_IMAGE, "--encoder", "RED", "-w", "10"]);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown encoder"));
}

#[test]
fn ramp_is_case_sensitive() {
    // Uppercase ramp name should trigger fallback warning
    let output = run_txtr(&[TEST_IMAGE, "--ramp", "DENSE", "-w", "10"]);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown ramp"));
}
