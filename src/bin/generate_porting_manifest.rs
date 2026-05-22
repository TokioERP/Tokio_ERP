use std::env;
use std::fs;
use std::path::Path;
use std::process;

use tokio_erp::porting::{build_porting_manifest, manifest_to_json};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: generate_porting_manifest <source> <target> <output>");
        process::exit(2);
    }

    let result =
        build_porting_manifest(Path::new(&args[1]), Path::new(&args[2])).and_then(|manifest| {
            let json = manifest_to_json(&manifest).map_err(std::io::Error::other)?;
            fs::write(Path::new(&args[3]), json)
        });

    if let Err(error) = result {
        eprintln!("{error}");
        process::exit(1);
    }
}
