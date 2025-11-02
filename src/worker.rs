use crate::job::{busywork, Message};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run(jobs: usize) {
    let (tx, rx) = mpsc::channel();
    let mut handles = (0..jobs)
        .map(|id| {
            let tx = tx.clone();
            let (t2, r2) = mpsc::channel();
            (
                t2,
                thread::spawn(move || {
                    busywork(id, tx, r2);
                }),
            )
        })
        .enumerate()
        .collect::<HashMap<_, _>>();

    let mut blockers = (0..jobs)
        .map(|id| HashSet::new())
        .enumerate()
        .collect::<HashMap<_, _>>();

    while handles.len() > 0 {
        let result = rx.recv_timeout(Duration::from_secs(75)).expect("Deadlock");
        match result {
            Message::Ok(job) | Message::Err(job) => {
                // println!("[{job}] success");
                let (_, handle) = handles.remove(&job).unwrap();
                handle.join().unwrap();

                let blocked_tasks = blockers.get_mut(&job).unwrap();
                blocked_tasks.iter().for_each(|blocked| {
                    let (tx, _) = handles.get_mut(blocked).unwrap();
                    tx.send(Message::Unblocked).unwrap();
                });
                blocked_tasks.clear();
                // println!("remaining tasks: {:?}", handles.keys());
            }
            Message::Blocked(job, blocker) => {
                // println!("[{job}] blocked by {blocker}");

                if handles.contains_key(&blocker) && !blockers[&job].contains(&blocker) {
                    // TODO detect cycles
                    blockers.get_mut(&blocker).unwrap().insert(job);
                } else {
                    let (tx, _) = handles.get_mut(&job).unwrap();
                    tx.send(Message::Unblocked).unwrap();
                }
            }
            Message::Unblocked => unreachable!(),
        }
    }

    println!("finished all tasks");
}
