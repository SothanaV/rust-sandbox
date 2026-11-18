use std::env;

fn main() {
    // Collect command-line arguments into a growable vector of Strings
    let args: Vec<String> = env::args().collect();

    // args[0] is always the compiled binary path (just like Python's sys.argv[0])
    println!("Binary path: {}", args[0]);

    if args.len() < 3 {
        eprintln!("Usage: {} <port> <precision>", args[0]); // Prints to stderr
        std::process::exit(1);
    }

    let port: u16 = args[1].parse().expect("Port must be a valid u16");
    let precision = &args[2];

    println!("Starting worker on port {port} with {precision} quantization");
}