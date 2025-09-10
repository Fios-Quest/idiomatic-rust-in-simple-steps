use async_rust::fake_work::ThreadedFakeWorker;
use async_rust::thread_executor::block_thread_on;
use std::time::{Duration, Instant};
use async_rust::join::Join;

fn main() {
    let timer_1_fut = async {
        ThreadedFakeWorker::new(Duration::from_secs(2)).await;
        println!("Timer 1 complete");
    };
    let timer_2_fut = async {
        ThreadedFakeWorker::new(Duration::from_secs(1)).await;
        println!("Timer 2 complete");
    };

    let fut = async {
        let now = Instant::now();
        let _ = Join::new(timer_1_fut, timer_2_fut).await;
        now.elapsed().as_millis()
    };

    let time_taken = block_thread_on(fut);
    println!("Time taken: {time_taken}ms")
}
