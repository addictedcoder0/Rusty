fn main() {
    println!("2+3 = {:?}",Add::add(2,3));
    println!("2.5+3.5 = {:?}",Add::add(2.5,3.5));
}



pub trait Add {
	fn add(Self , rhs : Self) -> Self;
}

impl Add for f32 {
	fn add<T :Add> (a:T,b:T) -> T{
	return a+b;
	}
}

impl Add for i32 {
	fn add<T :Add> (a:T,b:T) -> T{
	return a+b;
	}
}