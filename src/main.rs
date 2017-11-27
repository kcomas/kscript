
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("a = @[1, \" \", 2]\n a[0] = \"test\"\n a >> 1") {
        eprintln!("Error {:?}", kerror);
    }
}
