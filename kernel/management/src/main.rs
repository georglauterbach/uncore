use anyhow::Context;

macro_rules! __execute_command {
	($command:expr) => {{
		log::trace!("Executing command `{}`", $command);
		let mut command: Vec<&str> = $command.split(" ").collect();
		execute_command(command.remove(0), command)?
	}};
}

fn main() -> anyhow::Result<()> {
	helper::logger::initialize(Some(log::Level::Trace));
	log::debug!("Build started");

	// calling `.unwrap()` here is fine since we know
	// this path definitely ends with `/management`
	let root_directory = env!("CARGO_MANIFEST_DIR")
		.strip_suffix("/management")
		.context("could not determine root directory by stripping suffix `management`")
		.unwrap();

	// parse the main `Cargo.toml` file
	let kernel_cargo_toml_path = String::from(root_directory) + "/Cargo.toml";
	let mut cargo_metadata_command = cargo_metadata::MetadataCommand::new();
	cargo_metadata_command.manifest_path(kernel_cargo_toml_path);
	let cargo_metadata = cargo_metadata_command
		.exec()
		.context("Could not extract Cargo metadata from Cargo.toml")?;

	// prepare all environment variables that we need during build-time
	let git_commit = __execute_command!("git rev-parse HEAD");
	let kernel_version: String = cargo_metadata.root_package().unwrap().version.to_string();
	let mut kernel_version_extended: String = kernel_version.clone();
	kernel_version_extended.push_str(&git_commit);

	let rust_host_toolchain = __execute_command!("rustc -vV");
	let rust_host_toolchain = rust_host_toolchain
		.lines()
		.filter(|line| {
			fancy_regex::Regex::new("host:.+")
				.unwrap()
				.is_match(line)
				.unwrap()
		})
		.collect::<Vec<&str>>()[0]
		.split(" ")
		.collect::<Vec<&str>>()[1]
		.to_owned();

	let build_environment_vars = [
		// (
		// 	"COMPILATION_DATE_AND_TIME",
		// 	execute_command("date", &["+%H:%M, %d %b %Y"])?,
		// ),
		("GIT_COMMIT", git_commit),
		("KERNEL_VERSION", kernel_version),
		("KERNEL_VERSION_EXTENDED", kernel_version_extended),
		("RUST_HOST_TOOLCHAIN", rust_host_toolchain),
		// ("RUST_TARGET", execute_command("date", &[])?),
		// ("RUST_RUSTC_VERSION", execute_command("date", &[])?),
	];

	const ENV_PREFIX: &str = "__BUILD__";
	for (name, val) in build_environment_vars {
		println!("{}{}='{}'", ENV_PREFIX, name, val);
	}

	Ok(())
}

fn execute_command(program: &str, arguments: Vec<&str>) -> anyhow::Result<String> {
	let x = std::process::Command::new(program).args(arguments).output()?;
	let mut output = String::from_utf8(x.stdout).unwrap();
	output = output.trim().into();
	Ok(output)
}
