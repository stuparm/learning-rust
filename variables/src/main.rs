fn main() {
    // const HOUR: u32 = 60*60;
    // const THREE_HOURS: u32 = 3 * HOUR;

    let mut x = 1;
    println!("{x}");

    x = 2;
    println!("{x}");


    let y = 3;
    {
        let y = y * 2;
        println!(" y: {y}");
    }

    println!("y: {y}");

    ///////////////
    // let mut guess:u8 = "42".parse().expect("Not a number!");
    let c = 'a';
    let a = c as u32;
    println!("a: {a}");

    let mut tup= (1, 2, 3);
    let (a, b, c) = tup;
    println!("a: {a}, b: {b}, c: {c}");
    tup.0 = 4;
    x = tup.0;
    println!("{x}");

    let mut b: [u8; 3] = [1, 2, 3];
    b[0]=4;
    let c = b[0];
    println!("b {c}");

    // functions

    another_function(5,'c');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    fn one() -> i32 {
        1
    }

    let x = one();

    println!("The value of x is: {x}");
    
    if x > 3 {
        println!("x is greater than 3");
    } else {
        println!("x is not greater than 3");
    }
    
    let m = if x > 3 {
        "x is greater than 3"
    } else {
        "x is not greater than 3"
    };
    println!("{m}");
    
    let mut counter = 0;
    let result = loop {
        counter+=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");


    let a = [10, 20, 30, 40, 50];
    for i in a {
        println!("The value is: {i}");
    }
    
    let f = fibonacci(2);
    println!("f: {f}");
}

fn another_function(a:i32, b:char) {
    println!("another function {a} {b}");
}

fn fibonacci(n:i32) -> i32 {
    if n < 1 {
        return n;
    }
    n + fibonacci(n-1)
}
