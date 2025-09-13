use async_rust::fake_work::ThreadedFakeWorker;
use async_rust::thread_executor::block_thread_on;
use std::time::{Duration, Instant};

fn main() {
    // We can also Pin a future by putting it in a Box
    let worker_1 = ThreadedFakeWorker::new(Duration::from_secs(2));
    let worker_2 = ThreadedFakeWorker::new(Duration::from_secs(1));

    let worker_1_wrapper = async {
        let now = Instant::now();
        worker_1.await;
        now.elapsed().as_millis()
    };
    let worker_2_wrapper = async {
        let now = Instant::now();
        worker_2.await;
        now.elapsed().as_millis()
    };

    let fut = async {
        let now = Instant::now();
        let time_1 = worker_1_wrapper.await;
        let time_2 = worker_2_wrapper.await;
        (time_1, time_2, now.elapsed().as_millis())
    };

    let (t1, t2, tt) = block_thread_on(fut);
    println!("Time for future 1: {t1}ms");
    println!("Time for future 2: {t2}ms");
    println!("Total time:        {tt}ms");
}
