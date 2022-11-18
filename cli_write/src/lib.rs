use std::io;
use std::io::Write;
use cli_shared::models::CliAguments;
pub fn loop_write(args: CliAguments) {
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

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */
