
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run(
        "a = @[3, 2, 1]; B = %[\"key\": \"value\"]; a[0] > 1; B[\"key\"] > 1",
    )
    {
        eprintln!("Error {:?}", kerror);
    }
}
