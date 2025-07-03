fn main() {
    let mut u = User {
        username: String::from("A"),
        email: String::from("B"),
        active: true,
    };
    
    // u.email = String::from("C");
    let uu = build_user(String::from(u.username), String::from(u.email));
    println!("{}", uu.username);
    
    let uuu = User {
        ..uu
    };
    println!("{}", uuu.username);
    
    let c = Color(2,2,4);
    println!("{}", c.0);
    
    let rec1 = Rectangle {
        width: dbg!(1*4),
        height: 4,
    };
    
    println!("{}", area(&rec1));
    println!("{}", rec1.height);
    
    println!("{rec1:#?}");
    dbg!(&rec1);
    
    println!("{}", rec1.area());
    
    let mut sq = Rectangle::square(5);
    sq.resize()
    
}

struct User {
    active: bool,
    username: String,
    email: String,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
    }
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn resize(&mut self) {
        self.width = self.width*2;
        self.height = self.height*2;
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}