use anyhow::Context;

use management::rush;

fn main() -> anyhow::Result<()>
{
	helper::logger::initialize(Some(log::Level::Trace));
	log::trace!("Management process started");

	let kernel_root_directory = env!("CARGO_MANIFEST_DIR")
		.strip_suffix("/management")
		.context("could not determine root directory by stripping suffix `management`")?;

	// parse the main `Cargo.toml` file
	let cargo_metadata = cargo_metadata::MetadataCommand::new()
		.manifest_path(format! {"{}/Cargo.toml", kernel_root_directory})
		.exec()
		.context("Could not extract Cargo metadata from Cargo.toml")?;

	// prepare all environment variables that we need during build-time
	let git_commit = rush::execute("git rev-parse HEAD")?.stdout();
	let kernel_version: String = cargo_metadata.root_package().unwrap().version.to_string();
	let mut kernel_version_extended: String = kernel_version.clone();
	kernel_version_extended.push_str(&git_commit);

	let rust_host_toolchain = rush::execute("rustc -vV")?.stdout();
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
		// notify 'deb' 'Setting kernel environment variables'
		// 		COMPILATION_DATE_AND_TIME="$(date +'%H:%M, %d %b %Y')"
		//   GIT_REVISION_HEAD="$(git rev-parse --short HEAD)"
		//   KERNEL_VERSION="$(grep -m 1 'version*' Cargo.toml | cut -d '"' -f 2)"
		//   KERNEL_VERSION+=" (${GIT_REVISION_HEAD})"
		//   RUST_DEFAULT_TARGET="$(rustc -Vv | grep 'host:' | cut -d ' ' -f 2)"
		//   RUST_TOOLCHAIN="$(grep 'channel' rust-toolchain.toml | cut -d ' ' -f 3 | tr -d '"')"
		//   RUSTC_VERSION="$(rustc --version)" ; RUSTC_VERSION=${RUSTC_VERSION#rustc }
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

	rush::execute(format!("mkdir -p {}/build/qemu/kernel/EFI/BOOT/", kernel_root_directory))?;
	rush::execute(format!("mkdir -p {}/build/qemu/kernel/EFI/BOOT/", kernel_root_directory))?;
	rush::execute(format!(
		"mkdir -p {}/build/tests/kernel/EFI/BOOT/",
		kernel_root_directory
	))?;
	rush::execute(format!("mkdir -p {}/build/tests/boot_output/", kernel_root_directory))?;

	Ok(())
}
