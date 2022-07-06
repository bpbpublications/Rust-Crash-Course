fn main() {
    let sa = String::from("rust");
    println!("sa = {}", sa);
    println!("{:?}, {}, {}", sa.as_ptr(), sa.len(), sa.capacity());
    
    let sb = sa; // sa is assigned to sb
    
    println!("sb = {}", sb);
    println!("{:?}, {}, {}", sb.as_ptr(), sb.len(), sb.capacity());
}
