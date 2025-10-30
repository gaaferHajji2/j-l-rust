use clap::Parser;

/// A simple greeting program
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    println!("Hello, world!");

    let cli: Cli = Cli::parse();

    for _ in 0..cli.count {
        println!("Hello To Jafar Loka Simple App: {}", cli.name)
    }
}
