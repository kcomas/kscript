
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Void));
    if let Err(kerror) = kscript.run("a = ((1 + 2.5 - 2) * 4.123 / 2 + (10 % (4 + 1)) ^ 5)") {
        panic!("{:?}", kerror);
    }
}
