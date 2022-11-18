use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CliAguments {
    #[arg(short, long)]
    pub port: u64,
    #[arg(short, long)]
    pub ip: String,
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub mode: String,
}