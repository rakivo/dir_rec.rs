# dir_rec.rs

### You just copy [`dir_rec.rs`](https://github.com/rakivo/dir_rec.rs/blob/master/dir_rec.rs) file to your project and use it.

## Control the implementation of `Iterator` for `DirRec` with the `dir_rec_stop_on_error` feature.
> If you want to handle errors that may occur while iterating directories, turn `on` the `dir_rec_stop_on_error` feature,
> with the feature enabled, the `Iterator` implementation will return `Option<std::io::Result<PathBuf>>` type, allowing you to manage errors.

# [example.rs](https://github.com/rakivo/dir_rec.rs/blob/master/example.rs)
```rs
use std::env;
use std::process::ExitCode;

mod dir_rec;
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
```
