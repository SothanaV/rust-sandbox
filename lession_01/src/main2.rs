fn main() {
    let latency_ms = "12.5";
    // 1. Shadow `latency_ms` by parsing it into an `f32`
    let latency_ms: f32 = latency_ms.parse().unwrap();

    let batch_size: usize = 32;

    // 2. Fix the type mismatch below so Rust calculates requests per second
    let throughput = (batch_size as f32 / latency_ms) * 1000.0;

    println!("Throughput: {:.2} req/sec", throughput);
}