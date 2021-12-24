use std::{fs, io::Read};

use clap::App;

mod err;

fn main() {
    // NOTE(aleksey): cf. https://docs.rs/clap/2.34.0/clap/index.html
    let matches = App::new("gtfs-rt-validator")
        .version("0.1")
        .author("Aleksey Bilogur <aleksey.bilogur@gmail.com>")
        .about("Validates input GTFS-RT files against the specification.")
        .args_from_usage("<INPUT>             'Sets the input file to use'")
        .get_matches();

    let input = matches.value_of("INPUT");
    let input = match input {
        Some(_) => input.unwrap(),
        None => {
            let err_out = err::ValidationError::new(
                err::ErrKind::IOError,
                format!("Missing the required <INPUT> argument."),
            );
            println!("{}", err_out);
            return;
        }
    };

    let f = fs::File::open(input);
    let mut f = match f {
        Ok(_) => f.unwrap(),
        Err(e) => {
            let err_out = err::ValidationError::wrap(
                err::ErrKind::IOError,
                e,
                format!("Error opening the file '{}'", input),
            );
            println!("{}", err_out);
            return;
        }
    };
    let mut buf = Vec::new();
    match f.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(e) => {
            let err_out = err::ValidationError::wrap(
                err::ErrKind::IOError,
                e,
                format!("Error reading from the file '{}'", input),
            );
            println!("{}", err_out);
            return;
        }
    }

    let report = gtfs_rt_validator::report::ErrorReport::from_buffer(&mut buf);
    println!("{}", report);
}
