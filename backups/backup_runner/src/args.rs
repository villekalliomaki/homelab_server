use clap::Parser;
use log::info;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    #[arg(short, long, default_value = "backup_runner.toml")]
    pub config_file: String,
}

impl Args {
    pub fn generate() -> Args {
        info!("Getting arguments from command line");

        Args::parse()
    }
}
