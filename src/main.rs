
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("d=%[\"test\":{|d|(d=d+1);d}][\"test\"]|2|") {
        eprintln!("Error {:?}", kerror);
    }
}
