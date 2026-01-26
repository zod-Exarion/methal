use std::env;
use std::error::Error;
use std::fs;

fn main() {
    let content = match parse_args(env::args()) {
        Err(e) => {
            eprintln!("File content could not be recovered\n{e}");
            std::process::exit(1);
        }

        Ok(content) => content,
    };

    methal::run(content);
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<String, String> {
    args.next(); // skips the program name 

    let mut content = args
        .next()
        .ok_or("Invalid arguments\nEnter text or -f path")?;

    if content == "-f" {
        let path = args.next().ok_or("No file path provided")?;
        println!("{path}");
        content = read_file(path).map_err(|e| format!("File couldn't be read: {e}"))?;
    }

    Ok(content)
}

fn read_file(path: String) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
