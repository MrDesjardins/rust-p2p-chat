use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long)]
    port: u64,
    #[arg(short, long)]
    ip: String,
    #[arg(short, long)]
    username: String,
}

fn main() {
    let args = Cli::parse();
    println!("Rust P2P running on address {}:{} with username {}", args.ip, args.port, args.username);
}
