enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
struct Point{
	x:i32,
	y:i32,
}
fn main() {
//patterns :matching literals .
let x =1;
let x_str = match x {
	1 => "one",
	//matching multiple patterns :
	2|3 => "two or three",
	_ => "above three",
};
println!("x as string : {:?}",x_str );	
//
let c:char = 's';
// problem with matching :
// anything that introduces a new binding, they introduce shadowing. 
match c {
	// x will match all the cases, it is same as _ here.
	x => println!("char is : {:?}",x),
}
println!("x : {:?}",x );
// proper approach
match c {
	'c' => println!("char is c "),
	x => println!("char is {:?}",x),
}
// destructuring :
let point = Point{x:10,y:20};
match point{
	Point{x,y} => println!("x:{:?} ,y:{:?}",x,y),
}
match point{
	//mention only var , which you care for.
	// pattern uses .. to disregard the multiple values.
	Point{x,..} => println!("point of interest x:{:?}", x),
}
//ref and mutref 
match x{
	ref r => println!("got a ref of : {:?}",r ),
}
let mut y=1;
println!("befor modification y:{:?}",y );
match y{
	ref mut r => {*r =10;
					println!("modified value of y via ref:{:?}",r );},
}
println!("after modification y :{:?}", y);
//Ranges :
match y{
	1...5 => println!("small range"),
	_ => println!("big range"),
}
match c{
	'a'...'r' => println!("with in a to r range"),
	// binding with @
	e@'s'...'z' => println!("beyond r and char is :{:?}",e),
	_ => println!("something else"),
}
//Guards with "if"
let flag = false;
match y{
	e@1...5 if flag => println!("case 1:y :{:?} & flag:{:?}",e,flag),
	e@1...5 if !flag => println!("case 2:y :{:?} & flag:{:?}",e,flag),
	e@5...10 if flag => println!("case3:y :{:?} & flag:{:?}",e,flag),
	e@5...10 if !flag => println!("case4:y :{:?} & flag:{:?}",e,flag),
	_ => println!("no possible match"),
}
//match on Enums	
 let Move = Message::Move{x:10,y:20};
 let enum_variant = match Move {
 				Message :: Quit => "quit",
 				Message :: ChangeColor(r,g,b) => "ChangeColor",
 				Message :: Move {x,y}=> "move",
 				Message :: Write(s) => "write",
 				};
 println!("enum variant : {:?}",enum_variant);// "move"
}

