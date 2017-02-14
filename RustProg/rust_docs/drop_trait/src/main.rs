struct Explosive{
	strength:i32,
}
/* this trait is predefined don't define again.
trait Drop{
	fn drop(&mut self);
}*/
impl Drop for Explosive{
	fn drop(&mut self){
		println!("Strength of the Explosive is : {:?}",self.strength);
	}
}
fn main() {
	let fireCrackers:Explosive = Explosive{strength:100};
	let tnt : Explosive = Explosive{strength:1000};
	// do some stuff
}// at this point both of the var will go out of scope
