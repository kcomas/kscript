
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("a = 1; $?1==1^?2==35{a = (a + 1)} a > 1") {
        eprintln!("Error {:?}", kerror);
    }
}
