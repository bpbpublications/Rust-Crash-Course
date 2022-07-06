fn main() {
    let mut s = String::from("rust"); // s is mutable
    //let ref1 = &s;
    //let ref2 = &s;
    
    {
        let _r4 = &mut s;
    }
    let ref3 = &mut s;

    println!(" {}", ref3); 
} 
