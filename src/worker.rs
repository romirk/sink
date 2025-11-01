use crate::job::{busywork};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn run(jobs: usize) {
    let (tx, rx) = mpsc::channel();
    let mut handles = (0..jobs)
        .map(|id| {
            println!("starting worker thread");
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(busywork(id)).unwrap();
            })
        })
        .enumerate()
        .collect::<HashMap<_, _>>();

    while handles.len() > 0 {
        let result = rx.recv().unwrap();
        let id;
        match result {
            Ok(job) => {
                println!("[{job}] success");
                id = job;
            }
            Err(job) => {
                println!("[{job}] failed");
                id = job;
            }
        }

        let handle = handles.remove(&id).unwrap();
        handle.join().unwrap();
    }

    println!("finished all tasks");
}
