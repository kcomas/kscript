
mod kscript;

use self::kscript::Kscript;
use kscript::logger::{Logger, DebugLogger};

fn main() {
    let mut kscript = Kscript::new(DebugLogger::new());
    kscript.run("test = 1234;");
}
