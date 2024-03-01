//! `ginfo` is a simple utility to display information about a GZip header.
//! It can read from a file or from stdin, and can also read base64 encoded input.
//! It will display the compression method, flags, modification time, and OS of the GZip file.
//!
//! # Examples
//!
//! ```shell
//! $ ginfo test.gz
//! ```
//! ```shell
//! $ echo "H4sIAAAAAAAAA8vPUMhIzcnJVyjJSC1KBQBvyKZBDgAAAA==" | ginfo -b
//! ```
//! ```shell
//! $ ginfo -b test.b64
//! ```
//!

use base64::Engine;
use clap::{clap_derive, Parser};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};

// use clap to parse command line arguments
#[derive(clap_derive::Parser)]
struct Args {
    /// The filename to read from. If not provided, read from stdin.
    #[arg(last = true)]
    file: Option<String>,

    /// Read the input as base64 encoded.
    #[arg(short = 'b', long = "base64")]
    is_base64: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let header = match &args.file {
        Some(filename) => read_from_file(filename, args.is_base64)?,
        None => read_from_stdin(args.is_base64)?,
    };

    let id1 = header[0];
    let id2 = header[1];
    let cm = header[2];
    let flg = header[3];
    let mtime = u32::from_le_bytes(header[4..8].try_into().unwrap());
    let os = header[9];

    if id1 == 0x1f && id2 == 0x8b {
        println!("Valid GZip file.");
        println!("Compression Method: {}", cm);
        println!("Flags: {:08b}", flg);
        let time = UNIX_EPOCH + Duration::from_secs(mtime.into());
        let datetime: DateTime<Utc> = time.into();
        println!(
            "Modification Time: {}",
            datetime.format("%Y-%m-%d %H:%M:%S")
        );
        print_os(os);
    } else {
        println!("Not a valid GZip file.");
    }

    Ok(())
}

fn read_from_stdin(is_base64: bool) -> Result<[u8; 10], io::Error> {
    if is_base64 {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(input)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let header = decoded[..10].try_into().unwrap();
        Ok(header)
    } else {
        let mut header = [0u8; 10];
        io::stdin().read_exact(&mut header)?;
        Ok(header)
    }
}

fn read_from_file(filename: &str, is_base64: bool) -> Result<[u8; 10], io::Error> {
    if is_base64 {
        let mut file = File::open(filename)?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        let input = input.trim();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(input)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let header = decoded[..10].try_into().unwrap();
        Ok(header)
    } else {
        let mut file = File::open(filename)?;
        let mut header = [0u8; 10];
        file.read_exact(&mut header)?;
        Ok(header)
    }
}

fn print_os(os: u8) {
    let os_name = match os {
        0 => "FAT filesystem (MS-DOS, Windows NT/9x)",
        1 => "Amiga",
        2 => "VMS (or OpenVMS)",
        3 => "Unix",
        4 => "VM/CMS",
        5 => "Atari TOS",
        6 => "HPFS filesystem (OS/2, NT)",
        7 => "Macintosh",
        8 => "Z-System",
        9 => "CP/M",
        10 => "TOPS-20",
        11 => "NTFS filesystem (Windows NT)",
        12 => "QDOS",
        13 => "Acorn RISCOS",
        255 => "unknown",
        _ => "other",
    };
    println!("OS: {}", os_name);
}
