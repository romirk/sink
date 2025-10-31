use std::thread::sleep;
use std::time::Duration;
use rand::random;

pub struct Job {
    id: u32
}
impl Job {
    pub fn new(id: u32) -> Self {
        Job { id }
    }

    pub fn run(&self) {
        // simulate random effort
        let runtime = random::<u8>() / 10;
        let delay = Duration::from_millis(random::<u8>() as u64 * 10);
        let interval = Duration::from_millis(random::<u8>() as u64 * 10);
        println!("[{}] start {} every {} after {}", self.id, runtime, interval.as_millis(), delay.as_millis());
        sleep(delay);
        for i in (0..runtime).rev() {
            println!("[{}] {}", self.id, i);
            sleep(interval);
        }
        println!("[{}] end", self.id);
    }
}