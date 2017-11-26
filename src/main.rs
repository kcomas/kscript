
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run(
        "a = {|b, c| b = @[1]; c}; a|@[\"herp\", 'derp', %[\"key\": 1]], (1 + 2 * 4)| >> 1",
    )
    {
        eprintln!("Error {:?}", kerror);
    }
}
