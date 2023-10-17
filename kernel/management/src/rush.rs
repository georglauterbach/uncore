#[derive(Debug, thiserror::Error)]
pub enum Error
{
	#[error("general I/O fault")]
	GeneralIo(#[from] ::std::io::Error),
	#[error("could not convert from UTF8")]
	IoFromUtf8(#[from] ::std::string::FromUtf8Error),
}

pub trait GenericString: AsRef<::std::ffi::OsStr> + ::std::fmt::Display {}

impl<S: AsRef<::std::ffi::OsStr> + ::std::fmt::Display> GenericString for S {}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Command<S: GenericString>(S);

impl<S: GenericString> std::fmt::Display for Command<S>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl<'a> Into<Command<&'a str>> for &'a str
{
	fn into(self) -> Command<&'a str> { Command(self) }
}

impl Into<Command<String>> for String
{
	fn into(self) -> Command<String> { Command(self) }
}

struct Arguments<S: GenericString>(Vec<S>);

impl<S: ::std::fmt::Display + GenericString> ::std::fmt::Display for Arguments<S>
{
	fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		for element in &self.0 {
			write!(f, " {}", element)?;
		}

		Ok(())
	}
}

pub struct CommandInvocation<S: GenericString>
{
	command:      Command<S>,
	arguments:    Arguments<S>,
	_environment: Vec<(String, String)>,
}

impl<S: GenericString> CommandInvocation<S>
{
	fn new(command: impl Into<Command<S>>, arguments: Vec<S>) -> Self
	{
		Self {
			command:      command.into(),
			arguments:    Arguments(arguments),
			_environment: Vec::new(),
		}
	}

	pub fn execute(self) -> Result<CommandResult, Error>
	{
		log::trace!("Executing command '{}{}'", self.command, self.arguments);
		let x = std::process::Command::new(self.command.0)
			.args(self.arguments.0)
			.output()?;

		Ok(CommandResult {
			stdout:    String::from_utf8(x.stdout)?.trim().into(),
			stderr:    String::from_utf8(x.stderr)?.trim().into(),
			exit_code: x.status,
		})
	}
}

pub fn create_new_command(invocation: impl AsRef<str>) -> CommandInvocation<String>
{
	let mut invocation_vec: Vec<String> = invocation.as_ref().split(' ').map(String::from).collect();
	CommandInvocation::new(invocation_vec.remove(0), invocation_vec)
}

pub fn execute(invocation: impl AsRef<str>) -> Result<CommandResult, Error>
{
	let mut invocation_vec: Vec<&str> = invocation.as_ref().split(' ').collect();
	let invocation = CommandInvocation::new(invocation_vec.remove(0), invocation_vec);
	invocation.execute()
}

pub struct CommandResult
{
	stdout:    String,
	stderr:    String,
	exit_code: std::process::ExitStatus,
}

impl<'a> CommandResult
{
	pub fn stdout(self) -> String { self.stdout }

	pub fn stderr(self) -> String { self.stderr }

	pub fn stdout_and_err(self) -> (String, String) { (self.stdout, self.stderr) }

	pub fn status(&self) -> std::process::ExitStatus { self.exit_code }
}

