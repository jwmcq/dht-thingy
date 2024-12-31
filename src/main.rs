use clap::Parser;
use crate::sensors::temperature;

pub mod sensors;

#[derive(Parser)]
struct Cli {
    pin: u32,
    port: u32
}

fn main() {
    let args = Cli::parse();
    temperature::read(args.pin);

    println!("pin: {:?} port {:?}", args.pin, args.port);
}
