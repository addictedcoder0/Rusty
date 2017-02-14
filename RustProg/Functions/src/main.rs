fn main() {
    printme(add_one(5));
    printme(add_two(5));
    let x = add_one(10);
    printme(x);

    //    Function Pointers : We can also create variable bindings which point to functions
    let fao : fn (i32) -> i32 = add_one; 
    let y = fao(24);
    printme(y);

    diverge();
}

//you must declare the types of function arguments. 
fn printme(num:i32){
	println!("number is : {}", num);
}

//The last line of a function determines what it returns. You’ll note the lack of a semicolon here.
fn add_one(x:i32) -> i32 {
	x+1
}

/*
Rust is primarily an expression-based language. 
There are only two kinds of statements, and everything else is an expression.
Expressions return a value, and statements do not.

 in Rust the value of an assignment is an empty tuple () 
 because the assigned value can have only one owner, and any other returned value would be too surprising:

let mut y = 5;
let x = (y = 6);  // `x` has the value `()`, not `6`.

*/

//early returns
fn add_two(num:i32) ->i32 {
	return num+2;
}

// Diverging functions : functions that don't return any value
fn diverge() -> !{
	panic!("this function don't return any value ");
}
/*
If you want more information, you can get a backtrace by setting the RUST_BACKTRACE environment variable:
RUST_BACKTRACE=1 cargo run  => will provide the complete info about the failure
RUST_BACKTRACE=0 cargo run  => in order to forcefully shutting the BackTrack off.
*/




