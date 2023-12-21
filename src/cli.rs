use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Yet another CLI system monitor
pub struct Args {
    /// Show load of each CPU
    #[arg(short, long)]
    pub per_cpu: bool,

    /// Show memory sizes in gigabytes (MB is default)
    #[arg(short, long)]
    pub in_gb: bool
}