use crate::worker::run;
use std::num::NonZeroUsize;
use std::thread::{available_parallelism, sleep};
use std::time::Duration;

mod job;
mod worker;

fn main() {
    let cpu_count = available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get();
    // println!("{} jobs available", cpu_count);

    // sig::register_signal_handler(handle_signal);

    println!("\x1b[?1049h\x1b[?25l\x1b[0;0H     concurrency test");
    run(cpu_count);
    sleep(Duration::from_secs(5));
    print!("\x1b[?25h\x1b[?1049l");
}
