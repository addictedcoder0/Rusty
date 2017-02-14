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
		self::pub_function();//self addresses current module.
		self::pri_nested::pub_function();
		super::pub_function();//super addresses parent scope(outside module)
	}
	mod pri_nested{
		pub fn pub_function(){
			println!("nested_module:demo->pri_nested & fun: pub_function");
		}
		fn pri_function(){
			println!("nested_module:demo->pri_nested & fun: pri_function");
		}
	}
}
fn pub_function(){
	println!("i am out of module");
}
