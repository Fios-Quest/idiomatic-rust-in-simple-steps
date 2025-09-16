use async_rust::join::Join;
use async_rust::thread_executor::block_thread_on;
use async_rust::thread_timer::ThreadTimer;
use std::time::{Duration, Instant};

fn main() {
    let future_1 = async {
        let now = Instant::now();
        ThreadTimer::new(Duration::from_secs(2)).await;
        now.elapsed().as_millis()
    };
    let future_2 = async {
        let now = Instant::now();
        ThreadTimer::new(Duration::from_secs(1)).await;
        now.elapsed().as_millis()
    };

    let fut = async {
        let now = Instant::now();
        let (timer_1, timer_2) = Join::new(future_1, future_2).await.expect("Join failed");
        (timer_1, timer_2, now.elapsed().as_millis())
    };

    let (t1, t2, tt) = block_thread_on(fut);
    println!("Time for future 1: {t1}ms");
    println!("Time for future 2: {t2}ms");
    println!("Total time:        {tt}ms")
}
