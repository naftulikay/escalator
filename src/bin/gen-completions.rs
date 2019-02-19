use escalator::CLI;

use std::io;

use structopt::clap::Shell;
use structopt::StructOpt;

/// Generate shell completions for escalator.
#[derive(Debug, StructOpt)]
struct CompletionsCLI {
    /// The shell to generate completions for. Valid options are 'bash', 'zsh', 'fish',
    /// 'powershell' and 'elvish'.
    #[structopt(default_value="bash")]
    shell: Shell,
}

fn main() {
    let args = CompletionsCLI::from_args();

    let mut app = CLI::clap();
    app.gen_completions_to("escalator", args.shell, &mut io::stdout());
}
