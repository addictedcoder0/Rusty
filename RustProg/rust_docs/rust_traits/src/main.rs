use std::fmt::Debug;
#[derive(Debug)]
struct Circle{
	radius:f64,
}
#[derive(Debug)]
struct Square{
	side:f64,
}
// kind of interface
trait HasArea{
	fn area(&self)->f64 ;
fn purpose(&self){println!("i am a default method")}
}
impl HasArea for Circle{
	fn area(&self)->f64{
		std::f64::consts::PI * self.radius*self.radius
	}
}
impl HasArea for Square{
	fn area(&self)->f64{self.side * self.side}
}
// generic func on trait
//multiple trait bounds and where clause
fn print_area<T>(shape:T) where T:HasArea+Debug{
	println!("area is :{:?}",shape.area());
}
fn main() {
    let circle:Circle = Circle{radius:10.0};
    let square:Square = Square{side:10.0};
    circle.purpose();
    print_area(circle);
    print_area(square);
}
