extern crate gcc;

use std::process::Command;

const AYUMI_PATH: &'static str = "ayumi-lib/ayumi.c";
fn main() {
    // init submodules
    Command::new("git").arg("submodule").arg("update").output().unwrap_or_else(|e| {
        panic!("ayumi-lib build failed: {}", e);
    });
    // compile lib
    gcc::compile_library("libayumi.a", &[AYUMI_PATH]);
}
