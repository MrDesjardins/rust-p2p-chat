use cli_shared::models::CliAguments;
use clap::Parser;
use cli_write::loop_write;
use cli_read::loop_read;

fn main() {
    let args = CliAguments::parse();
    println!(
        "Rust P2P running [{}] on address {}:{} with username {}\n",
        args.mode, args.ip, args.port, args.username
    );

    match args.mode.as_str() {
        "w" => loop_write(args),
        "r" => loop_read(args),
        &_ => println!("Valid mode value are [w] to have a CLI in write mode or [r] for a CLI in read mode.")
    }
}
