use cli_shared::models::CliAguments;
pub fn loop_read(args: CliAguments) {
    println!("Read because of mode: {}", args.mode);
}