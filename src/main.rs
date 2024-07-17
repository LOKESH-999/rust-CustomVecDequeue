mod dequeue;
use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};
use std::sync::atomic::{AtomicBool,AtomicU64};
use std::thread;
use std::time::Duration;
// use std::l

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        READY.store(true, Relaxed); // Everything from before this store ..
        // const V:Vec<i32>=vec![1,2,34,5];
        thread::sleep(Duration::from_millis(50*3));
        DATA.store(123, Relaxed);
        // thread::sleep(Duration::from_millis(500));
        
    });
    while !READY.load(Relaxed) { // .. is visible after this loads `true`.
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));
}