
mod kscript;

use self::kscript::Kscript;
use kscript::logger::{Logger, DebugLogger, LoggerMode};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new(LoggerMode::Stdout));
    kscript.run("test = 1234;");
}
