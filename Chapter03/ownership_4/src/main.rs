fn main() {
    let sa = String::from("rust");
    let sb = sa.clone();
    println!("sa: {}, {:?}, {}, {}", sa, sa.as_ptr(), sa.len(), sa.capacity());
    println!("sb: {}, {:?}, {}, {}", sb, sb.as_ptr(), sb.len(), sb.capacity());

    let a = 10;
    let b = a;
    let c = b.clone();
    println!("{}, {}, {}", a, b, c);
}
