fn main() {

    let mut v:Vec<i32> = vec![1];

    let first = v.get(0).cloned();
    v.push(2);

    println!("{:?}", first);

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // let s = s1 + "-" + &s2 + "-" + &s3;


    use std::collections::HashMap;
    
    let mut scores : HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("test"), 10);
    let a = scores.get(&String::from("test")).unwrap().clone();
    let a = scores.get("test").unwrap();
    
    println!("{:?}", a);
    
    let v = vec![1, 2, 3];

    v[99];

    use std::fs::File;

    let r = File::open("./test.txt");
    match r {
        Ok(file) => println!("{:?}", file),
        Err(e) => println!("{:?}", e),
    }

    use std::io::ErrorKind;


    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    
    home.is_ipv4();
    
}
