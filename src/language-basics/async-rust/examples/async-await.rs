use async_rust::thread_executor::block_thread_on;
use async_rust::thread_timer::ThreadTimer;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

async fn calling_this_function_returns_a_future() -> String {
    String::from("Inside an async function")
}

fn main() {
    let this_block_is_a_future = async { String::from("Inside an async block") };
    println!("{}", block_thread_on(this_block_is_a_future));
    println!(
        "{}",
        block_thread_on(calling_this_function_returns_a_future())
    );

    // ---

    // This async block is Future<Output = &'static str>
    let future_with_value = async {
        ThreadTimer::new(Duration::from_secs(1)).await;
        "This future returns a static string reference"
    };

    // This async block is Future<Output = ()>
    let future_that_uses_the_value_from_the_other_future = async {
        let value = future_with_value.await;
        println!("Received: {value}");
    };

    // Running the second future so we can see the output
    block_thread_on(future_that_uses_the_value_from_the_other_future);
    
    // ---
    
    result_example();

    // ---
    
    let future = async {
        println!("Starting future"); // Only prints once
        ThreadTimer::new(Duration::from_secs(2)).await;
        ThreadTimer::new(Duration::from_secs(1)).await;
        println!("Ending future"); // Only prints once
    };

    block_thread_on(future);
    
    // ---

    let future = async {
        let now = Instant::now();
        ThreadTimer::new(Duration::from_secs(2)).await;
        ThreadTimer::new(Duration::from_secs(1)).await;
        now.elapsed().as_secs()
    };

    let time_taken = block_thread_on(future);
    println!("Time taken {time_taken} seconds");
}

async fn this_future_could_fail() -> Result<u64, String> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "Time went backwards... a lot!".to_string())?
        .as_secs();
    // Check if the seconds are a multiple of 2 and convert to Result
    time.is_multiple_of(2)
        .then_some(time)
        .ok_or_else(|| "A completely unforeseen thing happened!".to_string())
}

async fn use_fallible_future() -> Result<(), String> {
    let time = this_future_could_fail().await?; // <- gorgeous!
    println!("{time} secs have passed since the Unix Epoch");
    Ok(())
}

fn result_example() {
    match block_thread_on(use_fallible_future()) {
        Ok(()) => {}
        Err(message) => println!("Error: {message}"),
    }
}
