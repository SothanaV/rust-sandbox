/*
fn main(){
    let latencies: [f32; 5] = [14.2, 18.5, 9.8, 22.1, 11.4];
    let mut sum:f32 = 0.0;
    let mut count = 0;
    for latency in latencies {
        sum += latency;
        count += 1;
    }
    println!("sum : {} | count : {}", sum, count);
    let avg = sum / count as f32;
    println!("avg : {}", avg);

    let status = if avg <= 1 as f32 {"HEALTHY"} else {"DEGRADED"};
    println!("status : {}", status);
}
*/

// key
fn main() {
    let latencies: [f32; 5] = [14.2, 18.5, 9.8, 22.1, 11.4];
    let mut sum = 0.0;
    let mut breaches = 0;

    for latency in latencies {
        sum += latency;
        if latency > 15.0 {
            breaches += 1;
        }
    }

    let avg = sum / latencies.len() as f32;
    let status = if breaches <= 1 { "HEALTHY" } else { "DEGRADED" };

    println!("avg: {avg:.2} ms | breaches: {breaches} | status: {status}");
    // Output: avg: 15.20 ms | breaches: 2 | status: DEGRADED
}