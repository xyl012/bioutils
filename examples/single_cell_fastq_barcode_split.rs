#[derive(StructOpt)]
#[structopt(name = "dd_test", about = "Get disk read bandwidth.")]
struct Opt {
    #[structopt(short, default_value = "4096")]
    block_size: u64,
    #[structopt(short, default_value = "1")]
    thread_num: u64,
    #[structopt(short)]
    file: String,
}

use cmd_lib::*;
use std::io::Result;

fn main() {

}
let msg = "I love rust";
run_cmd!(du -ah . | sort -hr | head -n 10)?;
