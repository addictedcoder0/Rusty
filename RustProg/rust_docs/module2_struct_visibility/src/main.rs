use demo::nested::pub_function as nested_func;
mod demo{
	//public struct with public field
	pub struct WhiteBox<T>{
		pub contents :T,
	}
	//public struct with private field
	pub struct BlackBox<T>{
		contents:T,
	}
	impl<T> BlackBox<T>{
		// A public constructor
		pub fn new(contents:T)->BlackBox<T>{
			BlackBox{contents:contents,}
		} 
	}
	pub mod nested{
		pub fn pub_function(){
			println!("nested_module:demo->nested & fun: pub_function");
		}
	}
}
fn pub_function(){println!("i am not in module");}
fn main() {
    let white = demo::WhiteBox{contents:"WhiteBox is easily accessible"};
    println!("WhiteBox field contents : {:?}",white.contents);

    let Black = demo::BlackBox::new("BlackBox is accessible via constructor");
    //println!("BlackBox field contents : {:?}",Black.contents );=> error
    nested_func();
    {
    	use demo::nested::pub_function;
    	pub_function(); // nested pub_function will be invoked.
    }
    pub_function();
}


