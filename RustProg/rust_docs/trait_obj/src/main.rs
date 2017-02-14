trait Foo{
	fn method(&self) -> String;
}
impl Foo for u8{
	fn method(&self)->String{
		format!{"u8 : {:?}",*self}
	}
}
impl Foo for String{
	fn method(&self)->String{
		format!{"String : {:?}",*self}
	}
}
fn do_something_static<T>(v:T) where T:Foo{
	println!("{:?}",v.method());
}
fn do_something_dynamic(v:&Foo){
	println!("{:?}",v.method());
}
fn main() {
	let x:u8 = 5;
	let y:String = "hello-world".to_string();
//Dynamic Dispatch => trait objects
	do_something_dynamic(&x as &Foo);
	do_something_dynamic(&y);
//static dispatch => rust uses "monomorphization"
	do_something_static(x); // value is moved
	do_something_static(y); // value is moved
}
