fn main() {
    let batch = 32;
    let total_tokens = dbg!(batch * 128) + 64; // Prints `[src/main.rs:3] batch * 128 = 4096`
    let tag = format!("batch-{total_tokens}");
    println!("Tag: {tag}");
}