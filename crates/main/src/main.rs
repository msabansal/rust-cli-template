use clap::Parser;
use tracing::info;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    rust_cli_lib::tracing::init();

    let args = Args::parse();
    for _ in 0..args.count {
        info!("Hello {}!", args.name);
    }
}
