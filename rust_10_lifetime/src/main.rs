
fn main() {
    let str1 = String::from("hello");
    let str2 = String::from("world");
    let ans = longest_str(&str1, &str2);
    println!("{}", ans);
}

fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}