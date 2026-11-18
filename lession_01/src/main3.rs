fn main() {
    println!("{} {} {}", "-".repeat(10), "Tuples (Heterogeneous, Fixed-Length)", "-".repeat(10));
    /*
    Tuples (Heterogeneous, Fixed-Length)
    */
    // Type annotation: (&str, u16, f32)
    let endpoint = ("0.0.0.0", 8080, 0.95);

    // Destructuring (identical to Python: host, port, threshold = endpoint)
    let (host, port, threshold) = endpoint;

    // Index access via dot notation
    println!("Binding {}:{} (confidence >= {})", endpoint.0, endpoint.1, endpoint.2);
    println!("Unpacked: {host}:{port} @ {threshold}");

    // Unit tuple `()`: An empty tuple represents "no return value" (like Python's `None` return)
    let empty_return: () = ();

    /*
    Fixed-Size Arrays (Homogeneous, Stack-Allocated)
    */
    // Explicit type: [type; length]
    let rgb_pixel: [u8; 3] = [255, 128, 0];

    // Shorthand initialization: create an array of 512 zeros (great for fixed buffers)
    let mut zero_buffer: [f32; 512] = [0.0; 512];
    zero_buffer[0] = 1.0;

    // Indexing uses `usize`
    let first_channel = rgb_pixel[0];
    println!("First channel: {first_channel}, buffer len: {}", zero_buffer.len());

    // Bounds safety: Accessing an out-of-bounds index (e.g., rgb_pixel[5])
    // either fails at compile time or panics safely at runtime—never silent C-style memory corruption.


    /*
    Expressions vs. Statements (The Semicolon Rule)
    */
    let batch_tokens = {
        let prompt_len = 128;
        let max_new_tokens = 256;
        prompt_len + max_new_tokens // <-- No semicolon! Evaluates to 384
    }; // <-- Semicolon ends the `let` statement

    println!("Total context budget: {batch_tokens}");

    /*
    control flow
    */
    let use_fp8 = true;

    // Both branches must evaluate to the exact same type (&str)
    let dtype = if use_fp8 { "float8_e4m3fn" } else { "bfloat16" };
    println!("Selected dtype: {dtype}");



    /*
    Loop
    */
    // 1. `loop` returning a value on `break`
    let mut attempt = 0;
    let connected_port = loop {
        attempt += 1;
        if attempt == 3 {
            break 8080; // Exits the loop and returns 8080!
        }
    };
    println!("Connected on port {connected_port} after {attempt} attempts");

    // 2. `for` with ranges (`0..3` is exclusive: 0, 1, 2; `0..=3` is inclusive: 0, 1, 2, 3)
    let scores = [0.82, 0.91, 0.76, 0.95];
    let mut above_threshold = 0;

    for score in scores {
        if score >= 0.90 {
            above_threshold += 1;
        }
    }
    println!("High-confidence predictions: {above_threshold}");

    // 3. Iterating with index (just like Python's `enumerate(scores)`)
    for (idx, score) in scores.iter().enumerate() {
        println!("Item #{idx}: {score}");
    }
}