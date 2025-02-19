use std::env;
use std::process::ExitCode;

use dir_rec::DirRec; 

fn main() -> ExitCode {
    let args = env::args().collect::<Vec::<_>>();
    if args.len() < 2 {
        eprintln!("usage: {program} <directory to traverse recursively>", program = args[0]);
        return ExitCode::FAILURE
    }

    let ref dir_path = args[1];
    let dir = DirRec::new(dir_path);

    for file_path in dir.into_iter() {
        println!("{entry}", entry = file_path.display());
    }

    ExitCode::SUCCESS
}
