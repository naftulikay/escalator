use std::path::PathBuf;

use structopt::StructOpt;

/// Escalate user and group ids to root and execute a binary with arguments in place without forking.
#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::TrailingVarArg"))]
pub struct CLI {
    /// Verbosity; pass multiple times to increase log verbosity. By default, verbosity is set to
    /// ERROR, -v yields WARN, -vv yields INFO, -vvv yields DEBUG, and -vvvv yields TRACE.
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u8,
    /// A fully-qualified path to the binary to execute.
    #[structopt(parse(from_os_str))]
    pub binary: PathBuf,
    /// A list of arguments to pass to the binary upon execution.
    pub args: Vec<String>,
}
