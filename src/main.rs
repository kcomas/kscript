
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run(
        "a = @[2, 5]; b = {|| 4}; c = %[\"t\": 2, \"g\": 1]; d = (a[1] + b|| + c[\"g\"]); d >> 1",
    )
    {
        eprintln!("Error {:?}", kerror);
    }
}
