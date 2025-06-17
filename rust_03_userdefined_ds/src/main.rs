// Struct
struct Rect {
    height: f32,
    width: f32,
}

impl Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    // This will act like a static function
    fn print_something() {
        println!("Hey There");
    }
}

// Enum
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let r = Rect {
        width: 23.0,
        height: 10.0,
    };

    println!("{}, {}", r.width, r.height);
    println!("{}", r.area());
    Rect::print_something();

    let my_dir = Direction::North;
    steer(my_dir);
    let my_dir = Direction::South;
    steer(my_dir);
    let my_dir = Direction::West;
    steer(my_dir);
    let my_dir = Direction::East;
    steer(my_dir);
}

fn steer(dir: Direction) {
    // Pattern Matching
    match dir {
        Direction::East => println!("East Direction"),
        Direction::North => println!("North Direction"),
        Direction::West => println!("West Direction"),
        Direction::South => println!("South Direction"),
        _ => println!("Wrong Choice"),
    }
}
