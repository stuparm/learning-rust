fn main() {
    const HOUR: u32 = 60*60;
    const THREE_HOURS: u32 = 3 * HOUR;
    
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
}
