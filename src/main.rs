
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("py3 = 3; 23a = 3.12; 1S3 = 4") {
        eprintln!("Error {:?}", kerror);
    }
}
