#[derive(Clone, Copy)]

struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.width * self.height;
    }
}

fn main() {
    display_elements(1, 5);
    display_elements(String::from("Hello"), String::from("Hii"));

    let r = Rect {
        width: 10,
        height: 10,
    };

    let r1 = Rect {
        width: 10.2,
        height: 34.1,
    };

    println!("{}", r.area());
    println!("{}", r1.area());
}

fn display_elements<T: std::fmt::Display>(a: T, b: T) {
    println!("{}", a);
    println!("{}", b);
}
