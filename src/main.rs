use std::fmt::Display;

use clap::Parser;
use rouille::Response;
use crate::sensors::temperature;

pub mod sensors;


fn format_response<T: Display, U: Display>(temp: T, hum: U) -> String {
   format!("\
# TYPE temperature gauge
# TYPE humidity gauge
# UNIT temperature degrees_c
# UNIT humidity per_cent
temperature {}
humidity {}
# EOF", temp, hum)
}

#[derive(Parser)]
struct Cli {
    pin: usize,
    port: u32
}

fn main() {
    let args = Cli::parse();
    let addr = format!("0.0.0.0:{}", args.port);
    rouille::start_server(addr, move |_| {
        match temperature::read(args.pin) {
            Ok(rdg) => Response::text(format_response(rdg.temperature(), rdg.humidity())),
            Err(e) => {
                println!("{}", e);
                return Response::text(e.as_str()).with_status_code(500)
            }
        }
    })
}
