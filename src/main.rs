use escalator::CLI;

use log::LevelFilter;
use log::{debug, error, info};

use log4rs;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::Append;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::Encode;

use parking_lot::{Once, ONCE_INIT};

use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;
use std::process;

use structopt::StructOpt;

use nix::unistd::{execv, setgid, setuid, Gid, Uid};

/// Exit code to return when the binary to execute is not found.
static EXIT_CODE_COMMAND_NOT_FOUND: i32 = 127;
/// Static initializer guard for one-time initialization work.
static INITIALIZE: Once = ONCE_INIT;
/// Pattern for logging output.
static LOGGING_FORMAT: &'static str = "{l:5.5}: {m}{n}";

fn main() {
    let input_args = CLI::from_args();

    // initialize logging; no need for this level of safety, but why the hell not
    INITIALIZE.call_once(|| configure_logging(input_args.verbosity));

    if !input_args.binary.is_file() {
        error!(
            "Binary {} does not exist or is not a file.",
            input_args.binary.display()
        );
        process::exit(EXIT_CODE_COMMAND_NOT_FOUND);
    }

    if let Err(e) = setuid(Uid::from_raw(0)) {
        error!("Unable to assume the root user id via setuid: {}", e);
        process::exit(1);
    }

    if let Err(e) = setgid(Gid::from_raw(0)) {
        error!("Unable to assume the root group id via setgid: {}", e);
        process::exit(1);
    }

    // it's acceptable to unwrap here; CString::new returns an error if the input contains a \0 in
    // the middle, this won't happen under normal operation.
    let executable = CString::new(input_args.binary.as_os_str().to_owned().into_vec())
        .expect("Invalid NUL character in binary path.");

    let args: Vec<CString> = input_args
        .args
        .iter()
        .map(|s| CString::new(s.as_str()).expect("Invalid NUL character in arguments."))
        .collect();

    let argv: Vec<CString> = [executable.clone()].iter().chain(args.iter()).map(|i| i.to_owned()).collect();

    info!("Successfully assumed UID 0 and GID 0, executing with argv: {:?}; current process id: {}", argv, process::id());

    // goodbye cruel world
    execv(&executable, argv.as_slice()).unwrap();
}

fn configure_logging(verbosity: u8) {
    let encoder: Box<Encode> = Box::new(PatternEncoder::new(LOGGING_FORMAT));
    let appender: Box<Append> = Box::new(
        ConsoleAppender::builder()
            .encoder(encoder)
            .target(Target::Stderr)
            .build(),
    );

    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log4rs::init_config(
        Config::builder()
            .appender(Appender::builder().build("stderr", appender))
            .build(Root::builder().appender("stderr").build(level))
            .unwrap(),
    )
    .unwrap();

    debug!("Logging initialized.");
}
