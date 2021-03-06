// TODO: rewrite filetime
extern crate filetime;

use filetime::{set_file_times, FileTime};
use std::env;
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

const MAN_PAGE: &'static str = /* @MANSTART{touch} */ r#"
NAME
    touch - create file(s)

SYNOPSIS
    touch [ -h | --help ] FILE...

DESCRIPTION
    Create the FILE(s) arguments provided

OPTIONS
    -h
    --help
        display this help and exit
"#; /* @MANEND */

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stderr = stderr();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            let _ = stderr.write(b"Please provide an argument\n");
            exit(1);
        }
        _ => match args[1].as_str() {
            "-h" | "--help" => {
                if let Err(e) = stdout.write(format!("{}\n", MAN_PAGE).as_bytes()) {
                    let _ = stderr.write(format!("{}\n", e).as_bytes());
                    exit(1);
                };
                exit(0);
            }
            _ => {
                for arg in &args[1..] {
                    if Path::new(&arg).is_file() {
                        let time = FileTime::from_system_time(SystemTime::now());
                        if let Err(e) = set_file_times(&arg, time, time) {
                            let _ = stderr.write(format!("{}\n", e).as_bytes());
                        }
                    } else {
                        if let Err(e) = File::create(&arg) {
                            let _ = stderr.write(format!("{}\n", e).as_bytes());
                        }
                    }
                }
            }
        },
    };
}
