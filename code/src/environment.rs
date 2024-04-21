// SPDX-License-Identifier: GPL-3.0-or-later

//! This module provides functions to work efficiently with environment variables.

/// This constant stores the path of the directory containing the workspace's root
/// `Cargo.toml`.
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// Returns the version of `rustc` used for compiling `unCORE`.
fn get_rustc_version() -> anyhow::Result<String> {
  Ok(
    String::from_utf8(
      std::process::Command::new("rustc")
        .arg("--version")
        .output()?
        .stdout,
    )?
    .trim()
    .to_string(),
  )
}

/// Returns the kernels version as specified in `Cargo.toml`.
fn get_kernel_version() -> anyhow::Result<String> {
  let cargo_toml =
    std::fs::read_to_string(CARGO_MANIFEST_DIR.to_string() + "/uncore/Cargo.toml")?.parse::<toml::Table>()?;

  Ok(
    cargo_toml
      .get("package")
      .expect("Could not get table 'package' from unCORE's Cargo.toml")
      .get("version")
      .expect("Could not get value 'version' in table 'package' from unCORE's Cargo.toml")
      .to_string(),
  )
}

/// Returns the kernels version as specified in `Cargo.toml`.
fn get_toolchain() -> anyhow::Result<String> {
  let rust_toolchain_file = std::fs::read_to_string(CARGO_MANIFEST_DIR.to_string() + "/rust-toolchain.toml")?
    .parse::<toml::Table>()?;

  Ok(
    rust_toolchain_file
      .get("toolchain")
      .expect("Could not get table 'toolchain' from rust-toolchain.toml")
      .get("targets")
      .unwrap_or(&toml::Value::Array(vec!["riscv64gc-unknown-none-elf".into()]))
      .as_array()
      .expect("Could not convert array 'targets' in table 'package' to proper array")
      .first()
      .expect("Could not get first element of toolchain array")
      .to_string(),
  )
}

/// Returns a [`std::collections::HashMap`] that contains environment variables names as
/// keys and their respective values are the values of the map. This is used when
/// building, as the map is provided to [`std::process::Command`].
pub fn get_all_environment_variables_for_build(
  linker_script_path: &str,
) -> anyhow::Result<std::collections::HashMap<&'static str, String>> {
  let mut environment = std::collections::HashMap::new();

  environment.insert("RUSTC_VERSION", get_rustc_version()?);
  environment.insert("KERNEL_VERSION", get_kernel_version()?);
  environment.insert("RUST_TOOLCHAIN", get_toolchain()?);
  environment.insert("LOG_LEVEL", log::max_level().to_string());
  environment.insert(
    "COMPILATION_DATE_AND_TIME",
    chrono::offset::Local::now().format("%+").to_string(),
  );

  environment.insert("RUSTFLAGS", format!("-C link-arg=-T{linker_script_path}"));

  Ok(environment)
}

/// Returns `true` if we are building and running `unCORE` inside a Development Container.
pub fn is_inside_container() -> bool { std::env::var("UNCORE_DEV_CONTAINER").is_ok_and(|val| val == "true") }
