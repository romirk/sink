use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub type JobResult = Result<usize, usize>;
#[derive(Clone)]
pub struct Job {
    id: usize,
}
impl Job {
    pub fn new(id: usize) -> Self {
        Job { id }
    }

    pub fn run(&self) -> JobResult {
        let mut rng = rand::rng();

        // simulate random effort
        let runtime = rng.random::<u8>() / 10;
        let delay = Duration::from_millis(rng.random::<u8>() as u64 * 10);
        let interval = Duration::from_millis(rng.random::<u8>() as u64 * 10);

        println!(
            "[{}] start {} every {} after {}",
            self.id,
            runtime,
            interval.as_millis(),
            delay.as_millis()
        );
        sleep(delay);
        for i in (0..runtime).rev() {
            let should_error = rng.random::<u8>() == 0;
            if should_error {
                println!("[{}] stop {}", self.id, i);
                return Err(self.id);
            }
            println!("[{}] {}", self.id, i);
            sleep(interval);
        }
        println!("[{}] end", self.id);
        Ok(self.id)
    }
}
