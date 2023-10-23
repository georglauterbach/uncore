// SPDX-License-Identifier: GPL-3.0-or-later

//! This module provides functions to work efficiently with environment variables.

fn get_rustc_version() -> anyhow::Result<String> {
  Ok(String::from_utf8(
    std::process::Command::new("rustc")
      .arg("--version")
      .output()?
      .stdout,
  )?.trim().to_string())
}

#[must_use]
pub fn get_all_environment_variables() -> anyhow::Result<std::collections::HashMap<&'static str, String>> {
  let mut environment = std::collections::HashMap::new();
  environment.insert("RUSTC_VERSION", get_rustc_version()?);
  environment.insert(
    "COMPILATION_DATE_AND_TIME",
    chrono::offset::Local::now().format("%+").to_string(),
  );
  environment.insert("KERNEL_VERSION", "unknown".to_string());
  // environment.insert("LOG_LEVEL", log::max_level().to_string());
  environment.insert("LOG_LEVEL", "trace".to_string());

  Ok(environment)
}
