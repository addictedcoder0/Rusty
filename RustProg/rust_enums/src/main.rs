#[derive(Debug)]
enum compass{
	North,South,East,West
}
type species = &'static str;
#[derive(Debug)]
enum PlanetaryMonster{
	VenusMonster(species,i32),
	MarsMonster(species,i32),
	JupitorMonster(species,i32)
}
fn main() {
    println!("Direction {:?}",compass::East );
    //enums' value can be of other types or structs .
    //Enums are sometimes called Union Type or algebraic data type
    let kraken =  PlanetaryMonster::MarsMonster("kraken",200);
    let mutor = PlanetaryMonster::VenusMonster("mutor",150);  
    let raptor = PlanetaryMonster::JupitorMonster("raptor",430);
    println!("mars Monster : {:?}",kraken);
}
