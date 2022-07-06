struct Circle {
    cx: i32,
    cy: i32,
    r: f64
}
trait ShapeUtils {
    fn print_shape(&self);
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Implementing the ShapeUtils trait on Circle struct
impl ShapeUtils for Circle{
    fn print_shape(&self) {
        println!("Circle : [c = ({},{}), r = {}]", self.cx, self.cy, self.r);
    }
    fn area(&self) -> f64{
        3.14*self.r*self.r
    }
    fn perimeter(&self) -> f64{
        2.0*3.14*self.r
    }
}

fn main() {
    let shape1 = Circle{
        cx: 10,
        cy: 20,
        r : 5.0
    };
    shape1.print_shape();
    println!("{}", shape1.area());
    println!("{}", shape1.perimeter());
}
