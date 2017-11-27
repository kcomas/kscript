
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(VoidLogger::new(LoggerMode::Void));
    if let Err(kerror) = kscript.run(
        "a = 1; b = 0; c = 12; $c=>0${@[b, \" \"] > 1; tmp = a; a = (a + b); b = tmp; c = (c - 1)} \"\" >> 1") {
        eprintln!("Error {:?}", kerror);
    }
}
