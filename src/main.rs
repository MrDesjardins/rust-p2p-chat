use clap::Parser;
use std::io;
use std::io::Write;
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long)]
    port: u64,
    #[arg(short, long)]
    ip: String,
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    mode: String,
}

fn main() {
    let args = Cli::parse();
    println!(
        "Rust P2P running [{}] on address {}:{} with username {}\n",
        args.mode, args.ip, args.port, args.username
    );

    loop {
        let mut user_input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input_clean = user_input.trim();
        match user_input_clean {
            "exit" => break,
            "/name" => println!("Changing your name"),
            _ => println!("Command [{}] unrecognized", user_input_clean),
        }
    }
}
