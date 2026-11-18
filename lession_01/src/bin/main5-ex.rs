use std::io::{self, Write};

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
    print!("Enter threshold : ");
    io::stdout().flush().expect("err");

    let mut threshold_str = String::new();

    io::stdin().read_line(&mut threshold_str).expect("read threshold_str fail");
    println!("thresh read : {}", threshold_str);
    let data = [14.2, 18.5, 9.8, 22.1, 11.4];
    let thresh:f32 = threshold_str.trim().parse().expect("parse err");
    let (mean, breach_count, status) = evaluate_batch(data, thresh);
    println!("avg : {} | breach : {} => status : {}", mean, breach_count, status)
    
}