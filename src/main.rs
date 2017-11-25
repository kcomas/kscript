
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run(
        "d = %[\"asdf\": 1234, \"sub\":
                                     %[\"merp\": 3.45], \"arr\": @[1, 2, 4], \"herp\": \"derp\"]",
    )
    {
        eprintln!("Error {:?}", kerror);
    }
}
