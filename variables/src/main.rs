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
}
