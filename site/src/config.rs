use std::string::String;

/// gets a value from an environment variable and returns it.
/// if this call was mandatory and it couldn't get a value, it will exit
/// the program and write an error message.
pub fn get_from_env(variable: &str, mandatory: bool) -> String {
    std::env::var(variable).unwrap_or_else(|_| {
        if mandatory {
            println!("Error, couldn't read environment variable: {}", variable);
            std::process::exit(1);
        } else {
            panic!("Error, couldn't read environment variable: {}", variable);
        }
    })
}
