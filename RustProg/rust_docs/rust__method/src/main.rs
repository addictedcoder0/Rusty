#[derive(Debug)]
struct Circle{
	radius:f64,
}
//associated function :fn that don't take "self" parameter.
impl Circle{
	fn new(r:f64)->Circle{
		Circle{radius:r}
	}
}
impl Circle{
	fn area(&self) -> f64{
		//when the last line is the return value don't put ;
		std::f64::consts::PI * (self.radius)*(self.radius)
	}
	fn grow(&self,x:f64) -> Circle{
		Circle{radius:self.radius+x}
	}
	fn reference(&self){
		println!("taking self by reference");
	}
	fn mut_reference(&mut self){
		println!("taking self by mutable reference");
	}
	fn takes_ownership(self){
		println!("taking ownership of self");
	}
}
fn main() {
    let small_circle = Circle{radius:10.0};
    println!("small_circle :{:?}",small_circle );
    // ============== reference ====================
    //small_circle.reference();
    //small_circle.mut_reference(); : make small_circle mutable else error.
    //small_circle.takes_ownership(); : value will be moved
    println!("area of small_circle :{:?}",small_circle.area() );
    let medium_circle = small_circle.grow(5.0);
    println!("are of medium circle :{:?}",medium_circle.area() );
    println!("chained method call :{:?}", small_circle.grow(2.0).area());
    //Associated function .
    let big_circle = Circle::new(100.0);
    println!("big_circle :{:?}",big_circle );
    //Builder Design Pattern.
    let biggest_circle_area = Circle::new(100.0).grow(50.0).area();
    println!("biggest_circle_area :{:?}",biggest_circle_area );
}
