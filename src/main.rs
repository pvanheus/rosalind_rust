// Solutions for Project Rosalind problems
//
// Copyright (C) 2020  Peter van Heusden <pvh@webbedfeet.co.za>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::process::exit;

use project_rosalind::rosalind::*;

fn safe_open(filename: &str) -> BufReader<File> {
    BufReader::new(match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ERROR: Could not open {}: {}", filename, e);
            exit(1)
        }
    })
}

fn main() {
    let args = App::new("Project Rosalind")
        .version("0.1")
        .arg(Arg::with_name("PROBLEM").help("Which problem to run"))
        .arg(
            Arg::with_name("ARGS")
                .multiple(true)
                .help("Parameters for the problem to work on (e.g. files)"),
        )
        .get_matches();

    match args.value_of("PROBLEM") {
        Some(problem) => match problem {
            "ba1a" => {
                let filename = args.value_of("ARGS").unwrap();
                let lines = safe_open(filename)
                    .lines()
                    .collect::<Vec<Result<String, Error>>>();
                let dna = lines[0].as_ref().unwrap().as_bytes();
                let pattern = lines[1].as_ref().unwrap().as_bytes();
                let result = ba1a(dna, pattern);
                println!("{}", result);
            }
            _ => eprintln!("ERROR: Unknown problem: {}", problem),
        },
        None => eprintln!("ERROR: You must specify a problem to work on"),
    }
}
