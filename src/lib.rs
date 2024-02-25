use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Rust cat CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short = 'n',
        long = "lines",
        value_name = "LINES",
        default_value_t = 10
    )]
    lines: usize,

    /// Number of bytes
    #[arg(
        short = 'b',
        long = "bytes",
        value_name = "BYTES",
        value_parser,
        conflicts_with = "lines"
    )]
    bytes: Option<usize>,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                if let Some(num_bytes) = args.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

// fn parse_positive_int(val: &str) -> MyResult<usize> {
//     match val.parse() {
//         Ok(n) if n > 0 => Ok(n),
//         _ => Err(val.into()),
//     }
// }

// #[test]
// fn test_parse_positive_int() {
//     // 3は生の整数なのでOK
//     let res = parse_positive_int("3");
//     assert!(res.is_ok());
//     assert_eq!(res.unwrap(), 3);

//     // 数字でない文字列の場合はエラー
//     let res = parse_positive_int("foo");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

//     // 0もエラー
//     let res = parse_positive_int("0");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }
