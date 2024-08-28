fn main() {
    // This is how panic is manually calle
    panic!("Hello, world!");

    let nums: Vec<u64> = vec![12, 3, 45];

    // This will panic becuase we are accessing an array out of bounds.
    println!("{}", nums[20]);
    // We can run using RUST_BACKTRACE=1 cargo run to see the backtrace
}
