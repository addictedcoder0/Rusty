// A module 'demo'
mod demo{
	// Items in modules default to private visibility
	fn pri_function(){
		println!("module:demo & fun: pri_function");
	}
	// using pub modifer to override the default visibility
	pub fn pub_function(){
		println!("module:demo & fun: pub_function");
	}
	// Items can access other items in the same module,
	pub fn indirect_access(){
		println!("module:demo & fun: indirect_access");
		pri_function();
	}
	//module can also be nested 
	pub mod nested{
		fn pri_function(){
			println!("nested_module:demo->nested & fun: pri_function");
		}
		pub fn pub_function(){
			println!("nested_module:demo->nested & fun: pub_function");
		}
	}
	mod pri_nested{
		fn pri_function(){
			println!("nested_module:demo->pri_nested & fun: pri_function");
		}
	}
}
fn independentfn(){
	println!("i am out of module");
}
fn main() {
    independentfn();
    demo::pub_function();
    demo::indirect_access();
    //demo::pri_function(); => error
    demo::nested::pub_function();
    //demo::nested::pri_function();=> error
    //demo::pri_nested::pri_function();=> error
}

