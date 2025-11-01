use crate::job::Job;
use crate::worker::Scheduler;
use std::num::NonZeroUsize;
use std::thread::available_parallelism;

mod job;
mod worker;

fn main() {
    let cpu_count = available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get();
    println!("{} jobs available", cpu_count);
    let mut scheduler = Scheduler::new(if cpu_count > 255 {
        255
    } else {
        cpu_count as u8
    });
    for i in 0..cpu_count {
        scheduler.enqueue(Job::new(i));
    }
    scheduler.run();
}
