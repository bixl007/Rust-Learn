fn main() {
    println!("Hello, world!");
    let ans: u32 = sum(45, 63);
    println!("{}", ans);
    let bool_ans = is_even(6);
    println!("{}", bool_ans);
    let bool_ans = is_even(5);
    println!("{}", bool_ans);

    // String
    let str1 = String::from("Hey There");
    println!("This is the string: {}", str1);

    // Vector

    let vector_arr = [1, 3, 4];
    println!("{:?}", vector_arr);

    // Conditionals
    let even_res = is_even(40);
    if even_res {
        println!("The numer is even");
    } else {
        println!("The number is not even");
    }

    // For loop
    for i in 0..100 {
        println!("{}", i);
    }

    // Mutability
    let mut name = String::from("Hey");
    name.push_str(" There");
    println!("{}", name);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

// Booleans
fn is_even(a: u32) -> bool {
    return a % 2 == 0;
}
