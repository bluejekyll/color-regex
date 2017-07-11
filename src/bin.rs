extern crate clap;
extern crate colored;
extern crate regex;

use std::io::{stdin, stdout, BufRead, Write};

use clap::*;
use colored::Colorize;
use regex::{Regex, RegexSet};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("green")
                .help("Regex to match for green")
                .short("g")
                .long("green")
                .default_value("INFO"),
        )
        .arg(
            Arg::with_name("yellow")
                .help("Regex to match for yellow")
                .short("y")
                .long("yellow")
                .default_value("WARNING"),
        )
        .arg(
            Arg::with_name("red")
                .help("Regex to match for red")
                .short("r")
                .long("red")
                .default_value("ERROR"),
        )
        .arg(
            Arg::with_name("ready")
                .help("Regex to search for to see if application is 'ready'")
                .short("d")
                .long("ready")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pre")
                .help("Chars to print in order (animated) until 'ready'")
                .short("p")
                .long("pre")
                .default_value("|/-\\"),
                //.default_value("\u{f250}\u{f251}\u{f252}\u{f253}"),
        )
        .arg(
            Arg::with_name("post")
                .help("Chars to print in order (animated) after 'ready'")
                .short("s")
                .long("post")
                .default_value("+"),
                //.default_value("\u{2764}"),
        )
        .get_matches();

    // maybe use regexSet here?
    let green = matches.value_of("green").expect("green not specified");
    let yellow = matches.value_of("yellow").expect("yellow not specified");
    let red = matches.value_of("red").expect("red not specified");

    let regexes = RegexSet::new(vec![green, yellow, red]).expect("bad color regex");

    let ready = matches.value_of("ready").map(|s| {
        Regex::new(s).expect("bad regex for ready")
    });
    let pre_chars = matches.value_of("pre").expect("pre not specified");
    let post_chars = matches.value_of("post").expect("post not specified");

    // shared buffer
    let mut buf = String::new();

    let std_in = stdin();
    let std_out = stdout();

    // locking once for performance reasons...
    let mut input = std_in.lock();
    let mut output = std_out.lock();
    let mut is_ready = false;
    let mut ready_chars = pre_chars.chars();

    while let Ok(length) = input.read_line(&mut buf) {
        if length == 0 {
            break;
        }

        let line = match regexes.matches(&buf) {
            ref m @ _ if m.matched(2) => buf.red().bold(),
            ref m @ _ if m.matched(1) => buf.yellow(),
            ref m @ _ if m.matched(0) => buf.green(),
            _ => buf.normal(),
        };

        let ready_char: Option<char> = if let Some(ready) = ready.as_ref() {
            let mut ch = ready_chars.next();

            if !is_ready {
                is_ready = ready.is_match(&buf);
                if ch.is_none() {
                    ready_chars = pre_chars.chars();
                    ch = ready_chars.next();
                }
            } else {
                if ch.is_none() {
                    ready_chars = post_chars.chars();
                    ch = ready_chars.next();
                }
            }

            ch
        } else {
            None
        };

        write!(&mut output, "{}{}", ready_char.unwrap_or(' '), line)
            .expect("error writing to standard out");
        buf.clear();
    }
}
