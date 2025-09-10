use async_rust::fake_work::ThreadedFakeWorker;
use async_rust::thread_executor::block_thread_on;
use std::time::{Duration, Instant};

fn main() {
    // We can also Pin a future by putting it in a Box
    let worker_1 = ThreadedFakeWorker::new(Duration::from_secs(2));
    let worker_2 = ThreadedFakeWorker::new(Duration::from_secs(1));

    let worker_1_wrapper = async {
        worker_1.await;
        println!("Timer 1 complete");
    };
    let worker_2_wrapper = async {
        worker_2.await;
        println!("Timer 2 complete");
    };

    let fut = async {
        let now = Instant::now();
        worker_1_wrapper.await;
        worker_2_wrapper.await;
        now.elapsed().as_millis()
    };

    let time_taken = block_thread_on(fut);
    println!("Time taken: {time_taken}ms")
}
