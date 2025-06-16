fn main() {
    println!("Hello, world!");
    let ans: u32 = sum(45, 63);
    println!("{}", ans);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}
