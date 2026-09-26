use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    Ok(())
}
