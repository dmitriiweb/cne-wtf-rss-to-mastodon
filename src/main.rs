pub mod config;

use clap::Parser;
use log::error;

use config::Config;

#[derive(Parser, Debug)]
struct CliArgs {
    // path to .toml config file
    #[arg(short, long)]
    config: String,
}

fn main() {
    env_logger::init();
    let cli_args = CliArgs::parse();
    let config = Config::new(&cli_args.config).unwrap_or_else(|err| {
        error!("Can't parse cli arguments: {}", err);
        std::process::exit(1);
    });
    println!("{:?}", config);
}
