use std::env;
use std::path::Path;

fn execute_echo(iter: env::Args) {
    for (index, argument) in iter.enumerate() {
        if index > 0 {
            print!(" ");
        }
        print!("{}", argument);
    }
    println!("");
}

fn main() {
    let mut args = env::args();
    match args.next() {
        None => {}
        Some(arg) => match Path::new(&arg).file_name() {
            None => {}
            Some(fname) => match fname.to_str() {
                None => {}
                Some(fname) => {
                    if fname == "echo" {
                        execute_echo(args)
                    }
                }
            },
        },
    }
}
