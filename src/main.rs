
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("c=@[@[2]];a=??1=={|a|a}|1|&?2==c[0][0]") {
        eprintln!("Error {:?}", kerror);
    }
}
