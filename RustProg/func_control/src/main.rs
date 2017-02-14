fn main() {
	let health = 50;
    let active = if health>40 {false}else {true};
    println!("am i active : {:?}",active );
    println!("num is {:?}",abso(health) );
}
fn abso(x:i32) -> (i32){
	if x>0 {x}
	else{-x}
}
