fn avg(datas: [f32;5]) -> f32 {
    let mut count = 0;
    let mut sum:f32 = 0.0;
    for data in datas{
        count += 1;
        sum += data;
    }
    (sum as f32) / (count as f32)
}

fn is_sla_breach(latency_ms: f32, threshold_ms: f32) -> bool {
    latency_ms > threshold_ms
}

fn evaluate_batch(latencies: [f32; 5], threshold_ms: f32) -> (f32, usize, &'static str) {
    let mean = avg(latencies);
    let mut breach_count:usize = 0;
    for latency in latencies{
        breach_count += is_sla_breach(latency, threshold_ms) as usize;
    }
    let status = if breach_count <= 1 { "HEALTHY" } else { "DEGRADED" }; 

    (mean, breach_count, status)
}

fn main() {
    let data = [14.2, 18.5, 9.8, 22.1, 11.4];
    let (mean, breach_count, status) = evaluate_batch(data, 20.0);
    println!("avg : {} | breach : {} => status : {}", mean, breach_count, status)
}