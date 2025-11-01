use std::num::NonZeroUsize;
use std::thread::available_parallelism;
use crate::worker::run;

mod job;
mod worker;


fn handle_signal(sig: u8) {
    
}
fn main() {
    let cpu_count = available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get();
    println!("{} jobs available", cpu_count);
    
    // sig::register_signal_handler(handle_signal);

    run(cpu_count);
}
