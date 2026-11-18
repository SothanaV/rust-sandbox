fn classify_http_status(code: u16) -> &'static str {
    match code {
        200..=299 => "Success",
        400 | 422 => "Bad Payload / Schema Error", // `|` matches multiple patterns (OR)
        401 | 403 => "Auth Error",
        429 => "Rate Limited",
        500..=599 => "Upstream Server Error",
        _ => "Other Status", // `_` is required because `u16` has 65,536 possible values
    }
}

fn main() {
    println!("200 -> {}", classify_http_status(200));
    println!("422 -> {}", classify_http_status(422));
    println!("503 -> {}", classify_http_status(503));
}