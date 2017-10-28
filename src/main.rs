
mod kscript;

use self::kscript::Kscript;
use kscript::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    if let Err(kerror) = kscript.run("test = 1234") {
        panic!("{:?}", kerror);
    }
}
