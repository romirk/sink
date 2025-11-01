use crate::job::Job;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

struct Worker;

impl Worker {
    fn work(job: Job) {
        job.run();
    }
}

pub struct Scheduler {
    jobs: Vec<Job>,
    workers: Vec<Worker>,
}

impl Scheduler {
    pub fn new(workers: u8) -> Self {
        let jobs = Vec::new();
        let workers = (0..workers).map(|_| Worker {}).collect::<Vec<_>>();
        Scheduler { jobs, workers }
    }

    pub fn enqueue(&mut self, job: Job) {
        self.jobs.push(job);
    }
    pub fn run(self) {
        let (tx, rx) = mpsc::channel();
        let jobs = self.jobs.clone();
        let mut handles = jobs
            .into_iter()
            .map(|job| {
                println!("starting worker thread");
                let tx = tx.clone();
                thread::spawn(move || {
                    tx.send(job.run()).unwrap();
                })
            })
            .enumerate()
            .collect::<HashMap<_, _>>();

        while handles.len() > 0 {
            let result = rx.recv().unwrap();
            let id;
            match result {
                Ok(job) => {
                    println!("[{}] success", job);
                    id = job;
                }
                Err(job) => {
                    println!("[{}] failed", job);
                    id = job;
                }
            }

            let handle = handles.remove(&id).unwrap();
            handle.join().unwrap();
        }

        println!("finished all tasks");
    }
}
