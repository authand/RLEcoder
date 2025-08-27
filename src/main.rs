use std::env;
use std::process;
use std::fs::File;
use std::io::{self, stderr, stdout, BufReader, Read, Write};
use std::error::Error;

fn print_progress(percent: u64) {
   eprint!("\rProgress: {}%    ", percent);
   stderr().flush().unwrap(); 
}

fn encode(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let total_bytes = file.metadata()?.len();
    let reader = BufReader::new(file);

    let mut bytes_processed: u64 = 0;
    let mut last_percent: u64 = 0;

    let mut bytes = reader.bytes().peekable();
    let mut handle = stdout().lock();
 
    while let Some(byte_result) = bytes.next() {
        let current_byte = byte_result?;
        bytes_processed += 1;
        let mut count: u8 = 1;

        while let Some(Ok(next_byte)) = bytes.peek() {
            if *next_byte == current_byte && count < 255 {
                bytes.next();
                bytes_processed += 1;
                count += 1;
            } else {
                break;
            }
        }

        handle.write_all(&[count, current_byte])?;

        if total_bytes > 0 {
            let current_percent = (bytes_processed * 100) / total_bytes;
            if current_percent > last_percent {
                print_progress(current_percent);
                last_percent = current_percent;
            }
        }
    }

    print_progress(100);
    eprintln!();
    handle.flush()?;
    Ok(())
}

fn decode(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let total_bytes = file.metadata()?.len();
    let mut reader = BufReader::new(file).bytes();

    let mut bytes_processed: u64 = 0;
    let mut last_percent: u64 = 0;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(count_result) = reader.next() {
        let count = count_result?;
        bytes_processed += 1;

        // handle malformed files (maybe not RLE encoded file)
        let value = match reader.next() {
            Some(b) => {
                bytes_processed += 1;
                b?
            }
            None => return Err("Unexpected EOF: no value after count".into()),
        };

        for _ in 0..count {
            handle.write_all(&[value])?;
        }

        if total_bytes > 0 {
            let current_percent = (bytes_processed * 100) / total_bytes;
            if current_percent > last_percent {
                print_progress(current_percent);
                last_percent = current_percent;
            }
        }
    }

    print_progress(100);
    eprintln!();
    Ok(())
}
fn print_help() {
    println!("USAGE: ./RLEcoder [FLAG] [FILENAME] > [FILENAME]");
    println!("\nFlags:");
    println!("  -e        Encode the specified file.");
    println!("  -d        Decode the specified file.");
    println!("  --help    Show help message.");
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--help")) || args.contains(&String::from("-help")) {
        print_help();
        process::exit(0);
    }

    if args.len() != 3 {
        eprintln!("ERROR: Incorrect number of args.");
        print_help();
        process::exit(1);
    }

    let flag = &args[1];
    let filename = &args[2];

    match flag.as_str() {
        "-e" => {
            eprintln!("Encoding file: {}", filename);
            encode(&filename)?;
        }
        "-d" => {
            eprintln!("Decoding file: {}", filename);
            decode(&filename)?;
        }
        _ => {
            eprintln!("ERROR: Unknown flag '{}'", flag);
            print_help();
            process::exit(1);
        }
    };

    Ok(())
}