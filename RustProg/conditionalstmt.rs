
fn main(){
	println!("!true ={}",!true );
	println!("true || false = {}",true || false);
	println!("true != false = {}",true!=false );

	let age:i32 = 30;
	let value = if(age >25){"mature"}else {"immature"};
	println!("value : {}",value);

	if(age<10) { 
		println!("kidoo", );
	}else {
		println!("adult");
	}
}