#[derive(clap::Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub remaining: Remaining,
}

#[derive(clap::Parser, Debug)]
pub enum Commands {
    #[command(name = "--", disable_help_subcommand = true, disable_help_flag = true)]
    /// Forward all following arguments directly to Typst
    Defer(Remaining),
    /// Preview a specified file. Runs `watch`, writing output to `/tmp/` and opening it in the default viewer
    Preview(Remaining),
    /// (Re-) Discover installed Typst binaries
    Discover,
    /// Information on previously discovered binaries
    List,
}

#[derive(clap::Parser, Debug)]
pub struct Remaining {
    #[arg(allow_hyphen_values = true, num_args = 0..)]
    /// Tries to parse the first as a version string (i.e. `0.15`, or `0.13.1`), forwards others to Typst
    pub remaining: Vec<String>,
}
