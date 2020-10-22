use std::string::String;

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
