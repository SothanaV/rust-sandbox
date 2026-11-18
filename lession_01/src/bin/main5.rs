use std::io::{self, Write}; // Import `io` module and `Write` trait (needed for `.flush()`)

fn main() {
    print!("Enter target latency SLA (ms): ");
    // `print!` buffers stdout until a newline; `.flush()` forces immediate display
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new(); // Heap-allocated growable string buffer

    io::stdin()
        .read_line(&mut input) // Appends user input (including trailing `\n`) into `input`
        .expect("Failed to read line from stdin");

    // Shadow `input` to trim `\n` and parse into `f32`
    let sla_ms: f32 = input
        .trim()
        .parse()
        .expect("Please enter a valid floating-point number");

    println!("Configured SLA threshold: {sla_ms:.1} ms");
}