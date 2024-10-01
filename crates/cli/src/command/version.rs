use crate::exitcode::EXIT_OK;

pub fn version() -> i32 {
    let version = env!("CARGO_PKG_VERSION");
    println!("Version {version}");
    EXIT_OK
}
