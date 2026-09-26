use std::env;

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
    files: Vec<String>,
    pattern: String,
    case_insensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        // index 0 argument is always the one thats running the command
        if args.len() < 3 {
            return Err(format!(
                "Usage: {} <pattern> <file> (or <files> -i).",
                args[0]
            ));
        }

        let mut case_insensitive = false;
        let mut non_flag_args: Vec<String> = Vec::new();

        for arg in &args[1..] {
            if arg.starts_with("-") {
                match arg.as_str() {
                    "-i" => {
                        case_insensitive = true;
                    }
                    _ => {
                        return Err(format!("Error: flag {} doesn't exist.", arg));
                    }
                }
            } else {
                non_flag_args.push(arg.clone());
            }
        }

        if non_flag_args.len() < 2 {
            return Err(format!(
                "Usage: {} <pattern> <file> (or <files> -i).",
                args[0]
            ));
        }

        let pattern = non_flag_args[0].clone();
        let files: Vec<String> = non_flag_args[1..].to_vec();

        Ok(Config {
            files,
            pattern,
            case_insensitive,
        })
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;

    dbg!(&config);

    Ok(())
}
