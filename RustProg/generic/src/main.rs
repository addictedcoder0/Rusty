struct Pair<T> {
	first : T,
	second : T,
}
#[derive(Debug)]
struct Actor<T> {
	id: i32 ,
	name : &'static str,
	battle_gear : T,
}
fn main() {
    let number_pair:Pair<i32> = Pair{first:100,second:200};
println!("numbers :");
println!("first:{:?}\nsecond:{:?}",number_pair.first,number_pair.second);
	let magicians_pair:Pair<&str> = Pair{first:"xeron",second:"gandalf"};
println!("magicians :");	
println!("first:{:?}\nsecond:{:?}",magicians_pair.first,magicians_pair.second);
    let zin = Actor {id:23,name:"zin",battle_gear:"sword"};
    let krim = Actor {id:24,name:"krim",battle_gear:342};
println!("fighter 1 : {:?} has battle_gear :{:?}",zin.name,zin.battle_gear);
println!("fighter 2 : {:?} has battle_power :{:?}",krim.name,krim.battle_gear);

// writing a custom func that returns Result/Option => good practice
match sq_root(4.00){
	Ok(root) => println!("root is {:?}",root ),
	Err(issue) => println!("{:?}",issue)
 };
match sq_root(-25.00){
	Ok(root) => println!("root is {:?}",root ),
	Err(issue) => println!("{:?}",issue)
} ;
}
fn sq_root(r:f32)->Result<f32,String>{
	if r< 0.0 {
		return Err("number can not be negative".to_string());
	}
	Ok((r as f32).sqrt())
}
