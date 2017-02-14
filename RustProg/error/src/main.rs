struct Alien{
	name:&'static str,
	health : u32,
	damage : u32
}
impl Alien{
	fn create(n:&'static str ,mut h:u32 , d:u32) -> Alien{
		//constraints
		if h > 100 {h=100;}
		Alien {name:n,health:h,damage:d}
	}
	fn message(&self){
		println!("people of earth,i am:{:?}..leave the planet else perish !!!",self.name );
	}
	fn attack(&self){
		println!("once {:?} attcks , he damages {:?}% of life",self.name,self.damage );
	}
}
fn main() {
	    let xeron:Alien = Alien::create("xeron",132,10);
	    xeron.message();
	    xeron.attack();
}
