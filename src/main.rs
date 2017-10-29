
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("a = (1 * ((2 + 4) % 2) + 1 ^ 5)") {
        panic!("{:?}", kerror);
    }
}
