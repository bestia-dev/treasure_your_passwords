// build_win_cli_bin_mod.rs

//! Functions to build a CLI binary executable for Windows - cross compile.

use crate::cl;

use cargo_auto_lib::CargoTomlPublicApiMethods;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

/// cargo build --release for x86_64-pc-windows-gnu
pub fn task_win_release()->cl::CargoToml {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo build --release --target x86_64-pc-windows-gnu").unwrap_or_else(|e| panic!("{e}"));
    cargo_toml
}
