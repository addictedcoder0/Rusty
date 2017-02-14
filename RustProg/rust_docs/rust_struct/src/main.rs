#[derive(Debug)]
struct Point{
	x:i32, // rust don't support mutability at field level; like mut x:i32
	y:i32
}
//but rust supports mutable references as fields
struct MutPointRef<'a>{
	x:&'a mut i32,
	y:&'a mut i32
}
//tuplestruct
#[derive(Debug)]
struct Color(i32,i32,i32);
fn main() {
    let point1 = Point{x:5,y:10};
    println!("point1 on map x:{:?} ,y:{:?}",point1.x,point1.y );
    let point2 = Point{x:12,..point1};// y will be copied from point1.
    println!("point2 on map x:{:?} ,y:{:?}",point2.x,point2.y );
    // all structs are immutable by default , by binding we can make it mutable 
    let mut mut_point = Point{x:10,y:20};
    mut_point.x=40;
    println!("mutable pointer {:?}",mut_point );
    println!("mut_points on map x:{:?} ,y:{:?}",mut_point.x,mut_point.y );
    let mut_point_ref = MutPointRef{x:&mut mut_point.x,y:&mut mut_point.y};
    *mut_point_ref.x = 100;
    println!("MutPointRef on map x:{:?} ,y:{:?}",mut_point_ref.x,mut_point_ref.y );
    let black = Color(0,0,0);
    let green = Color(0,255,0);
    let Color(r,g,b)=green;
    println!("black : {:?} , green :{:?}",black,green );
    println!("black components : r:{:?} ,g:{:?} ,b:{:?}",black.0,black.1,black.2 ); 
    println!("green components : r:{:?} ,g:{:?} ,b:{:?}",r,g,b );                  
}
