fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(" world");
        println!("{s}");
    }
    {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("{s2}");
    }
    {
        let mut s = String::from("hello 😄😄");  // s comes into scope
        change(&mut s); 
        println!("{s}");
        
        let len = string_length(&s);
        println!("{len}");
    }
    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("{r1}");
        }
        let r2 = &mut s;
        println!("{r2}");
    }
    {
        let reference_to_nothing = dangle();
    }
    {
        let mut s = String::from("hello");
        let word = first_word(&s);
        // s.clear();
        println!("{word}");
    }
    {
        let a = [1,2,3];
        let slice = &a[1..3];
        assert_eq!(*slice, [2,3])
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
}

fn change(val: &mut String) { // some_integer comes into scope
    val.push_str(" world2");
} 

fn string_length(s: &String) -> usize {
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}