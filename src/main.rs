use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::error::Error;

fn encode(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut bytes = reader.bytes().peekable();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
 
    while let Some(byte_result) = bytes.next() {
        let current_byte = byte_result?;
        let mut count: u8 = 1;

        while let Some(Ok(next_byte)) = bytes.peek() {
            if *next_byte == current_byte && count < 255 {
                bytes.next();
                count += 1;
            } else {
                break;
            }
        }

        handle.write_all(&[count, current_byte])?;
    }

    handle.flush()?;
    Ok(())
}

fn decode(path: &str) -> Result<(), Box<dyn Error>> {
// for byte_result in bytes {
//         let byte = byte_result?;

//         if prev_byte == byte {
//             count += 1
//         } else {
//             let repeated_bytes = vec![prev_byte; count];
//             handle.write_all(&repeated_bytes)?;

//             prev_byte = byte;
//             count = 1;
//         }
//     }

//     let repeated_bytes = vec![prev_byte; count];
//     handle.write_all(&repeated_bytes)?;
//     handle.flush()?;
    Ok(())
}
fn print_help() {
    println!("USAGE: ./RLEcoder [FLAG] [FILENAME]");
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