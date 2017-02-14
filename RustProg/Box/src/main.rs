/*struct A {b : B}
struct B {c : i32}

fn main(){
	let mut a = A {b : B {c : 2}};
	// as 'a' is mutable every element under its scope is mutable.
	a.b.c = 10;
	a.b = B{c : 25};
	a = A {b : B {c : 100}};

	let z = a; // immutable ref
	z.b.c = 200; //error 
}*/


use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::result::Result;

fn main(){
let (tx,rx):(Sender<usize>,Receiver<usize>) = mpsc::channel();
let handle;
	{
	let tx = tx.clone();
	handle=thread::spawn(move ||{expensive_task(& tx)});
	}
// in order to print the complete child thread sequence , 
//use join() method on spawn().join().
for j in 1..10
{
println!("receiving data num:{} and value is {}", j ,rx.recv().unwrap());
}
handle.join();
}

fn expensive_task(tx:& Sender<usize>){
	for i in 1..10
	{
		println!("pushing data num:{} to channel=> {:?}",i,2*i);
		tx.send(i*2);
	}
}



/*
fn main(){
// spwan a child thread to be run in parallel 
// now once the main thread is finish working , child thread will also be killed.
thread::spawn(||{
	expensive_task();
}).join();
// in order to print the complete child thread sequence , use join() method on spawn().join().
for j in 10..20
	{
		println!("{:?}",j );
	}
}

fn expensive_task(){
	for i in 1..10
	{
		println!("{:?}",i );
	}
}
*/
/*use std::rc::Rc;

fn main(){
	let data1 = Rc::new(3i); // reference count of 1
	{
		let data2 = data1.clone(); // reference count of 2
		work_with(data2); // transfer ownership of data2
	}// reference count of 1
	work_with(data1);// transfer ownership of data1
}//reference count of 0 , memory deallocated

fn work_with(data : Rc<int>) {}



*/
/*
fn main(){
	let mut a:Vec<i32> = vec![1,2,3];
	{
		//let b = &a; // freezes 'a'
		a.push(4);
	}
	a.push(6);
	println!("{:?}", a);
}

fn helper(num : &mut i32){
	*num+=1;
	println!("a : {}",*num );
}
*/

/*struct inner{x:i32}
fn main() {
	let x = inner {x:3};
    let y = helper(&x);
    println!("val:{}",y );//3,lifetime of y is same as x.
}

fn helper(s : &inner) ->&i32{
	s.clone();
	return &s.x;
}*/


//
//let a:&Vec<i32> ;
 /*    let slot = Box::new(3);
   
    helper1(&slot); // resource is borrowed here
    helper1(&slot); // resource is borrowed here*/



/*fn print(num:Vec<i32>){
	println!("num is {:?}",num );
}*/


/*
fn helper1(slot : &Box<i32>){
	println!("this is a borrowed reference , and num is {} ",*slot);
}*/




