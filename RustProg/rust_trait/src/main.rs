struct Alien{
	name:&'static str,
	planet:&'static str,
	damage:&'static str,
	weapon:&'static str
}
struct Zombie{
	country :&'static str,
	damage:&'static str,
	weapon:&'static str
}
trait Monster{
	fn message(&self);
	fn attack(&self);
}
impl Monster for Alien{
	fn message(&self){
		println!("i am Alien : {:?} from :{:?}",self.name,self.planet);
		println!("i can reduce your health by using : {:?}% ",self.damage);
	}
	fn attack(&self){
		println!("i attack using : {:?}",self.weapon);
	}
}
impl Monster for Zombie{
	fn message(&self){
		println!("i am Zombie from :{:?}",self.country);
		println!("i can reduce your health by {:?}% ",self.damage);
	}
	fn attack(&self){
		println!("i attack by : {:?}",self.weapon);
	}
}
fn main() {
    let xeron:Alien = Alien {name:"xeron",planet:"jupitor",damage:"10",weapon:"lazer-blade"};
    let z1:Zombie = Zombie{country:"U.S.A",damage:"5",weapon:"biting"};
    xeron.message(); xeron.attack();z1.message(); z1.attack();
}
