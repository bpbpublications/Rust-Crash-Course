fn main() {
    let sa = String::from("rust");
    println!("sa = {}", sa);
    println!("ptr = {:?}\nlength = {}\ncapacity = {}", 
            sa.as_ptr(), sa.len(), sa.capacity());
}
