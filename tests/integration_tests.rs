// In tests/commands.rs
mod common;
use common::TestEnv;
use std::str;

#[test]
fn test_general_workflow() {
    let test_env = TestEnv::new();

    // Init.
    let output = test_env.run_command(&["init"]);

    assert!(output.status.success());
    assert!(test_env.envoke_dir.exists());

    // Create.
    let output = test_env.run_command(&["create", "dev"]);
    assert!(output.status.success());
    assert!(test_env.envoke_path("dev").exists());

    let output = test_env.run_command(&["create", "dev"]);
    assert!(!output.status.success());

    // List profiles.
    let output = test_env.run_command(&["list"]);
    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("dev"));

    // Switch.
    let output = test_env.run_command(&["switch", "dev"]);
    println!("{:?}", output);
    assert!(output.status.success());

    // Current.
    let output = test_env.run_command(&["current"]);
    println!("{:?}", output);
    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout.trim(), "dev");

    let output = test_env.run_command(&["create", "prod"]);
    assert!(output.status.success());
    assert!(test_env.envoke_path("prod").exists());

    let output = test_env.run_command(&["switch", "prod"]);
    assert!(output.status.success());

    // Verify current profile changed.
    let output = test_env.run_command(&["current"]);
    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout.trim(), "prod");

    // Remove.
    let output = test_env.run_command(&["remove", "prod"]);
    assert!(output.status.success());
    assert!(!test_env.envoke_path("prod").exists());

    // There should now be no active profile as prod was removed.
    let output = test_env.run_command(&["current"]);
    assert!(!output.status.success());
}

#[test]
fn test_uninitialized_errors() {
    let test_env = TestEnv::new();

    // All commands except init should fail when uninitialized.
    let output = test_env.run_command(&["create", "dev"]);
    assert!(!output.status.success());

    let output = test_env.run_command(&["list"]);
    assert!(!output.status.success());

    let output = test_env.run_command(&["current"]);
    assert!(!output.status.success());

    let output = test_env.run_command(&["switch", "dev"]);
    assert!(!output.status.success());

    let output = test_env.run_command(&["remove", "dev"]);
    assert!(!output.status.success());
}

#[test]
fn test_switch_with_force() {
    let test_env = TestEnv::new();

    // Initialize.
    let output = test_env.run_command(&["init"]);
    assert!(output.status.success());

    // Create a profile.
    let output = test_env.run_command(&["create", "dev"]);
    assert!(output.status.success());

    // Create a regular file named .env (not a symlink).
    let env_path = test_env.temp_path().join(".env");
    std::fs::write(env_path, "regular file").unwrap();

    // Try to switch without force (should fail).
    let output = test_env.run_command(&["switch", "dev"]);
    assert!(!output.status.success());

    // Try to switch with force (should succeed).
    let output = test_env.run_command(&["switch", "dev", "--force"]);
    assert!(output.status.success());

    // Verify current profile.
    let output = test_env.run_command(&["current"]);
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout.trim(), "dev");
}

#[test]
fn test_remove_current_profile() {
    let test_env = TestEnv::new();

    // Initialize.
    let output = test_env.run_command(&["init"]);
    assert!(output.status.success());

    // Create and switch to a profile.
    let output = test_env.run_command(&["create", "dev"]);
    assert!(output.status.success());

    let output = test_env.run_command(&["switch", "dev"]);
    assert!(output.status.success());

    // Remove the current profile.
    let output = test_env.run_command(&["remove", "dev"]);
    assert!(output.status.success());

    // Verify no current profile.
    let output = test_env.run_command(&["current"]);
    assert!(!output.status.success());
}
