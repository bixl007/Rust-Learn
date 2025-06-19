trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    width: u32,
    height: u32,
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 10,
    };
    let ans = get_area(r);
    println!("Answer: {}", ans);
}

fn get_area(s: impl Shape) -> u32 {
    s.area()
}
