extern crate kscript;

use kscript::lang::Kscript;

fn main() {
    let mut kscript = Kscript::new();

    kscript.run_from_args().unwrap();
}
