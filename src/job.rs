use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub type JobResult = Result<usize, usize>;
/// Dummy worker
pub fn busywork(id: usize) -> JobResult {
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
            return Err(id);
        }
        println!("[{id}] {i}");
        sleep(interval);
    }

    println!("[{id}] end");
    Ok(id)
}