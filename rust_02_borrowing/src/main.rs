fn main() {
    let str = String::from("Hey");
    let len = get_length(&str);
    println!("{} {}", str, len);

    // Mutable & immutable referrence
    let mut s1 = String::from("Hey There");
    let s2 = &mut s1;
    s2.push_str(" Whassuppp");
    println!("{}", s2);
    let s3 = &s1;
    println!("{}", s3);
}

fn get_length(str: &String) -> usize {
    let len = str.len();
    return len;
}
