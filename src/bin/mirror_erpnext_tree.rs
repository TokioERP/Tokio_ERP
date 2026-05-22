use std::env;
use std::path::Path;
use std::process;

use tokio_erp::porting::mirror_directory_tree;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: mirror_erpnext_tree <source> <target>");
        process::exit(2);
    }

    match mirror_directory_tree(Path::new(&args[1]), Path::new(&args[2])) {
        Ok(created) => println!("created {created} directories"),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}
