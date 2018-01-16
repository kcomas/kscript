extern crate kscript;

use std::process;
use kscript::lang::Kscript;

fn main() {
    let mut kscript = Kscript::new();

    let exit_code = kscript.run_from_args().unwrap();

    process::exit(exit_code);
}
