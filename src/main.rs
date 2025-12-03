use acme_rust_template::add;
use clap::Parser;
use log::{debug, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// The main logic that can be benchmarked
pub fn run(name: &str, count: u8) -> i32 {
    info!("Running logic with name: {} and count: {}", name, count);
    
    let result = add(40, 2);
    println!("40 + 2 = {}", result);

    debug!("Running print loop");
    for i in 0..count {
        println!("Hello {}! (iteration {})", name, i + 1);
        debug!("Print iteration: {}", i);
    }

    result
}

fn main() {
    // Initialize the logger
    if let Err(e) = acme_rust_template::logger::init() {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    info!("Application started");

    let args = Args::parse();
    let result = run(&args.name, args.count);

    info!("Application finished successfully with result: {}", result);
}
