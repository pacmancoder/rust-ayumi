extern crate gcc;

use std::env;
use std::process::Command;

const AYUMI_PATH: &'static str = "ayumi-lib/ayumi.c";
fn main() {
    // init submodules
    Command::new("git").args(&["submodule", "update", "--init", "ayumi-lib"])
                       .current_dir(env::var("CARGO_MANIFEST_DIR").unwrap())
                       .output().unwrap();
    // compile lib
    gcc::compile_library("libayumi.a", &[AYUMI_PATH]);
}
