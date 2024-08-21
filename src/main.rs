#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn from(size: u64) -> Self {
        Sizes {
            bytes: format!("{} bytes", size),
            kilobytes: format!("{} kilobytes", size / 1_000),
            megabytes: format!("{} megabytes", size / 1_000_000),
            gigabytes: format!("{} gigabytes", size / 1_000_000_000),
        }
    }

    fn parse(input: &str) -> Result<Sizes, String> {
        let data = input.split_whitespace().collect::<Vec<&str>>();
        let size = match data[0].parse::<u64>() {
            Ok(n) => n,
            Err(_) => Err("Invalid number provided")?,
        };
        let unit = match data.get(1) {
            Some(s) => s,
            None => Err("No unit provided")?,
        };

        let bytes = match *unit {
            "bytes" => size,
            "kb" => size * 1_000,
            "mb" => size * 1_000_000,
            "gb" => size * 1_000_000_000,
            _ => Err(format!("Invalid unit provided: {}", size))?,
        };

        Ok(Sizes::from(bytes))
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} \"<file size>\"", args[0]);
        std::process::exit(1);
    }

    match Sizes::parse(args[1].as_str()) {
        Ok(sizes) => println!("{:?}", sizes),
        Err(e) => eprintln!("{}", e),
    }
}
