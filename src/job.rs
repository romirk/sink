use std::io::{stdout, Write};
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
    let runtime = 25;
    let delay = Duration::from_millis(rng.random::<u8>() as u64 * 10);
    let interval = Duration::from_millis(rng.random_range(0..500));

    // println!(
    //     "[{}] start {} every {} after {}",
    //     id,
    //     runtime,
    //     interval.as_millis(),
    //     delay.as_millis()
    // );

    let row = id + 3;
    sleep(delay);
    print!("\x1b[{row};0H[{id}]\x1b[{row};30H|");
    stdout().flush().expect("unable to flush");

    for i in (1..runtime).rev() {
        let col = 30 - i;
        let should_block = rng.random::<u8>() % 20 == 0;
        if should_block {
            let mut blocker = rng.random_range(0..10);
            while blocker == id {
                blocker = rng.random_range(0..10) ;
            }
            print!("\x1b[{row};32H{blocker}");
            stdout().flush().expect("unable to flush");
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
            print!("\x1b[{row};32H  ");
            stdout().flush().expect("unable to flush");
            // println!("[{id}] unblocked");
        }
        // println!("[{id}] {i}");


        print!("\x1b[{row};{col}H*");
        stdout().flush().expect("unable to flush");
        sleep(interval);
    }

    // println!("[{id}] end");
    tx.send(Message::Ok(id)).unwrap();
    Ok(id)
}