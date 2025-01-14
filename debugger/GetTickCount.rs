// Explanation:
// Timer-based Anti-Debugging:

// The Instant::now() function records the start time of the routine.
// The elapsed time is calculated after the routine finishes.
// If the elapsed time exceeds a certain threshold, it may indicate the presence of a debugger, which slows down execution due to single-stepping or breakpoints.
// Custom Threshold:

// The threshold_ms value can be adjusted based on how sensitive you want the detection to be.


use std::time::Instant;

fn is_being_debugged(threshold_ms: u128) -> bool {
    let start_time = Instant::now();

    // Simulate the routine you want to time
    std::thread::sleep(std::time::Duration::from_millis(10));

    let elapsed_time = start_time.elapsed().as_millis();
    if elapsed_time > threshold_ms {
        println!(
            "Routine took too long to execute: {} ms, possibly being debugged.",
            elapsed_time
        );
        return true;
    }
    false
}

fn main() {
    let threshold_ms = 5000; // Set your threshold
    if is_being_debugged(threshold_ms) {
        println!("Debugging detected!");
    } else {
        println!("No debugging detected.");
    }
}
