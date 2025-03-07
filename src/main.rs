use std::env;

use findtext_doc::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <keyword> <file.xlsx>", args[0]);
        return;
    }

    let keyword = &args[1];
    let filepath = &args[2];
    let ret = search(keyword, filepath);

    match ret {
        Ok(ret) => println!("{}", if ret { "Found." } else { "Missing." }),
        Err(err) => eprint!("{}", err),
    }
}
