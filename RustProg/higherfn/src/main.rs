fn main() {
println!("== power level ==");	
println!("Basic:{:?}",1000 );
println!("small stone: {:?}",triple_me(1000) );
println!("magic stone: {:?}",magic_triple(triple_me,1000));
// triple_me is very simple and basic , 
//it can be converted to closure .
let triples = |n|{3*n};
println!("== closure Based impl ==");
println!("small stone: {:?}",triples(1000) );
println!("magic stone: {:?}",magic_triple(triples,1000) );
}
fn triple_me(num:i32)->i32{
	num*3
}
// higher order function  : magic_triple(), 
//means it takes another funn as param
fn magic_triple<F: Fn(i32) -> i32 >(f:F,num:i32)->i32{
	f(f(num))
}
