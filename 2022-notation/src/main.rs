mod domain;
mod substance;
mod util;

use std::fmt::Debug;

use nom::{
    error::{convert_error, VerboseError},
    Finish,
};

fn show((remaining, stmts): (&str, Vec<impl Debug>)) {
    for stmt in stmts {
        println!("{:?}", stmt);
    }
    println!("{:?}", remaining);
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents: &str = &std::fs::read_to_string(&filename).unwrap();
    if filename.ends_with(".dsl") {
        match domain::parse::<VerboseError<&str>>(contents).finish() {
            Ok(v) => show(v),
            Err(e) => println!("{}", convert_error(contents, e)),
        }
    } else if filename.ends_with(".sub") {
        match substance::parse::<VerboseError<&str>>(contents).finish() {
            Ok(v) => show(v),
            Err(e) => println!("{}", convert_error(contents, e)),
        }
    } else {
        panic!();
    }
}
