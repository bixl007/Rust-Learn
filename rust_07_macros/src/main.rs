macro_rules! say_hello {
    () => {
        println!("Hello world");
    };
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    age: u32,
}

fn main() {
    let u = User {
        username: String::from("Hello"),
        password: String::from("dsjflsdjfl"),
        age: 20
    };

    print!("{:?}", u); // Debug
}
