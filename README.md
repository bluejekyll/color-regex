# color-regex

Colorize the output of a log. Quick and dirty tool for adding some color to logs

## Install

Download and install Rust: https://rustup.rs/

```console
$ cargo install
```

## Usage

Takes it's stdin and matches on specified strings to then colorize the output.

Get help with:

```console
$ crx --help
color-regex 0.1.0
Benjamin Fry <benjamin.fry@salesforce.com>


USAGE:
    crx [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --green <green>      String to match for green [default: INFO]
    -r, --red <red>          String to match for red [default: ERROR]
    -y, --yellow <yellow>    String to match for yellow [default: WARN]
```

Right now this only supports matching a single string. It's a very dumb `contains` match on the input string that it colorizes.

## Example

```console
$ cat sayonaradb.log | crx -r ERROR -y WARNING -g LOG
... lots of pretty colored logs...
```