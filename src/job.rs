use std::sync::mpsc::{Receiver, Sender};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub type JobResult = Result<usize, usize>;

pub enum Message {
    Ok(usize),
    Err(usize),
    Blocked(usize, usize),
    Unblocked,
}

/// Dummy worker
pub fn busywork(id: usize, tx: Sender<Message>, rx: Receiver<Message>) -> JobResult {
    let mut rng = rand::rng();

    // simulate random effort
    let runtime = rng.random::<u8>() / 10;
    let delay = Duration::from_millis(rng.random::<u8>() as u64 * 10);
    let interval = Duration::from_millis(rng.random::<u8>() as u64 * 10);

    println!(
        "[{}] start {} every {} after {}",
        id,
        runtime,
        interval.as_millis(),
        delay.as_millis()
    );

    sleep(delay);
    for i in (1..runtime).rev() {
        let should_error = rng.random::<u8>() == 0;
        if should_error {
            println!("[{id}] stop {i}");

            tx.send(Message::Err(id)).unwrap();
            return Err(id);
        }

        let should_block = rng.random::<u8>() % 10 == 0;
        if should_block {
            let mut blocker = rng.random_range(0..10);
            while blocker == id {
                blocker = rng.random_range(0..10) ;
            }
            tx.send(Message::Blocked(id, blocker)).unwrap();
            while let response = rx.recv().unwrap()  {
                match response {
                   Message::Unblocked => break,
                    _ => {
                        tx.send(Message::Err(id)).unwrap();
                        return Err(id);
                    }
                }
            }
            println!("[{id}] unblocked");
        }
        println!("[{id}] {i}");
        sleep(interval);
    }

    println!("[{id}] end");
    tx.send(Message::Ok(id)).unwrap();
    Ok(id)
}