use std::thread;
use crate::job::Job;

mod job;

fn main() {
    println!("Hello, world!");
    let jobs = (0..10).map(|i| thread::spawn(move || {
        let job = Job::new(i);
        job.run()
    })).collect::<Vec<_>>();
    for job in jobs {
        job.join().expect("Failed to join");
    }
}
