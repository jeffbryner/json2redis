extern crate redis;
use std::thread;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;
use redis::{Client, Commands};
use std::error::Error;
use std::io::{Write, stderr};

static NTHREADS: i32 = 10;


/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored.
fn print_error(mut err: &Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(cause) = err.cause() {
        let _ = writeln!(stderr(), "caused by: {}", cause);
        err = cause;
    }
}

fn run() -> io::Result<()> {
    let path="events.json";

    // threads
    let mut children = vec![];
    for _ in 0..NTHREADS{
        //make a thread
        children.push (thread::spawn(move || {

            let client = Client::open("redis://127.0.0.1/").unwrap();
            let conn = client.get_connection().unwrap();

            let input = File::open(path).unwrap();
            let buffered = BufReader::new(input);

            for line_result in buffered.lines() {
                //println!("Thread {}, pushes {}", i,line.unwrap());
                let line = line_result.unwrap();
                let _ : () = conn.rpush("eventqueue", line).unwrap();
            }
        }));
    }

    for child in children{
        let _ = child.join();
    }
    Ok(())
}

fn main() {
    //run().unwrap();
    if let Err(err) = run() {
        print_error(&err);
        std::process::exit(1);
    }
}
