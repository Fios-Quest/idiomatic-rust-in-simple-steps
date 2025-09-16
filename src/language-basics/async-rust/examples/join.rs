use std::error::Error;
use async_rust::join::Join;
use async_rust::thread_executor::block_thread_on;
use async_rust::thread_timer::ThreadTimer;
use std::time::{Duration, Instant};
use async_rust::join::collapsable_future::InnerFutureSpentError;

async fn run_both_timers() -> Result<u64, InnerFutureSpentError> {
    let now = Instant::now();
    Join::new(
        ThreadTimer::new(Duration::from_secs(2)),
        ThreadTimer::new(Duration::from_secs(1)),
    ).await?;
    Ok(now.elapsed().as_secs())
}

fn main() -> Result<(), Box<dyn Error>> {
    let time_taken = block_thread_on(run_both_timers())?;
    println!("Time taken {time_taken} seconds");
    Ok(())
}
