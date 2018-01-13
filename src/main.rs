extern crate kscript;

use std::process;
use kscript::lang::Kscript;

fn main() {
    let mut kscript = Kscript::new();

    let code = kscript.run_file("./examples/fib.ks").unwrap();

    process::exit(code);
}
