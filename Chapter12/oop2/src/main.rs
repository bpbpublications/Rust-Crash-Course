struct Rectangle{
    l: f64,
    w: f64
}
impl Rectangle{
    fn area(&self)->f64{
        self.l * self.w
    }
}

struct Triangle{
    a: f64,
    b: f64,
    c: f64
}
impl Triangle{
    fn area(&self)->f64{
        let s = (self.a + self.b + self.c)/2.0;
        (s*(s-self.a)*(s-self.b)*(s-self.c)).sqrt()
    }
}

struct Circle{
    r: f64
}
impl Circle{
    fn area(&self)->f64{
        3.14 * self.r * self.r
    }
}

fn foo_rectangle(r: Rectangle){
    let area = r.area();
    println!("area = {}", area);
}

fn foo_triangle(t: Triangle){
    let area = t.area();
    println!("area = {}", area);
}

fn foo_circle(c: Circle){
    let area = c.area();
    println!("area = {}", area);
}

fn main(){
    let r1 = Rectangle {l: 10.0, w: 20.0};
    foo_rectangle(r1);
    
    let t1 = Triangle {a: 3.0, b: 4.0, c: 5.0};
    foo_triangle(t1);
    
    let c1 = Circle {r: 5.0};
    foo_circle(c1);
}
