
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("d = @[1, @[3, 4], 3, 4][1][1]") {
        eprintln!("Error {:?}", kerror);
    }
}
