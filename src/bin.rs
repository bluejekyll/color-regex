extern crate clap;
extern crate colored;

use std::io::{stdin, stdout, BufRead, Write};

use clap::*;
use colored::Colorize;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("green")
                .help("String to match for green")
                .short("g")
                .long("green")
                .default_value("INFO"),
        )
        .arg(
            Arg::with_name("yellow")
                .help("String to match for yellow")
                .short("y")
                .long("yellow")
                .default_value("WARN"),
        )
        .arg(
            Arg::with_name("red")
                .help("String to match for red")
                .short("r")
                .long("red")
                .default_value("ERROR"),
        )
        .get_matches();

    let green = matches.value_of("green").expect("green not specified");
    let yellow = matches.value_of("yellow").expect("yellow not specified");
    let red = matches.value_of("red").expect("red not specified");

    // shared buffer
    let mut buf = String::new();

    let std_in = stdin();
    let std_out = stdout();

    // locking once for performance reasons...
    let mut input = std_in.lock();
    let mut output = std_out.lock();

    while let Ok(length) = input.read_line(&mut buf) {
        if length == 0 {
            break;
        }

        let line = match buf {
            ref s @ _ if buf.contains(green) => s.green(),
            ref s @ _ if buf.contains(yellow) => s.yellow(),
            ref s @ _ if buf.contains(red) => s.red().bold(),
            ref s @ _ => s.normal(),
        };

        writeln!(&mut output, "{}", line).expect("error writing to standard out");
        // output.write(line.as_bytes()).expect("error writing to standard out");
        // output.flush().expect("error flushing stdout");
        buf.clear();
    }
}
