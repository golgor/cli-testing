mod funcs;

use clap::Parser;
use funcs::testare;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args: Args = Args::parse();
    println!("Count: {}", args.count);
    let x: i32 = testare(2);
    println!("x: {}", x);

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}