use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "David Vernet <void@manifault.com>",
       version = "0.0.1",
       about = "A terminal-based system-monitoring tool written in rust.")
]
struct Cli {
    /// A sample arg for when I want to add arguments later.
    sample_arg: Option<String>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    log::info!("Running monocle: {}!", cli.sample_arg.unwrap_or(String::from("(o_O)7")));
}
