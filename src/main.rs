
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("c = {|a| a > 1; 5}; d = c|\"test\"|") {
        eprintln!("Error {:?}", kerror);
    }
}
