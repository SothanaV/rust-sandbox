
fn compute_throughput(tokens: usize, latency_ms: f32) -> f32 {
    // Implicit return: last expression WITHOUT a semicolon
    (tokens as f32 / latency_ms) * 1000.0
}

fn analyze_latencies(samples: [f32; 5]) -> (f32, usize) {
    let mut sum = 0.0;
    let mut breaches = 0;

    for s in samples {
        sum += s;
        if s > 15.0 {
            breaches += 1;
        }
    }

    (sum / samples.len() as f32, breaches)
}

fn main() {
    let tps = compute_throughput(512, 16.0);
    println!("Throughput: {tps:.1} tokens/sec");

    let data = [14.2, 18.5, 9.8, 22.1, 11.4];
    let (avg, breaches) = analyze_latencies(data);
    println!("Average: {avg:.2} ms, Breaches: {breaches}");
}