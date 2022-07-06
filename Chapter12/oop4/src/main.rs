trait Shape{
    fn area(&self)->f64;
}

struct Rectangle{
    l: f64,
    w: f64
}
impl Shape for Rectangle{
    fn area(&self)->f64{
        self.l * self.w
    }
}

struct Triangle{
    a: f64,
    b: f64,
    c: f64
}
impl Shape for Triangle{
    fn area(&self)->f64{
        let s = (self.a + self.b + self.c)/2.0;
        (s*(s-self.a)*(s-self.b)*(s-self.c)).sqrt()
    }
}

struct Circle{
    r: f64
}
impl Shape for Circle{
    fn area(&self)->f64{
        3.14 * self.r * self.r
    }
}

fn foo_dd(t: &dyn Shape){
    let area = t.area();
    println!("area = {}", area);
}

fn main(){
    let r1 = Rectangle {l: 10.0, w: 20.0};
    foo_dd(&r1);
    
    let t1 = Triangle {a: 3.0, b: 4.0, c: 5.0};
    foo_dd(&t1);
    
    let c1 = Circle {r: 5.0};
    foo_dd(&c1);
}
