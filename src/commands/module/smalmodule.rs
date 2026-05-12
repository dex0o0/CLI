use std::{thread, time::Duration};
pub fn sleep(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}
