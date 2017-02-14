#[derive(Debug)]
struct Player{
	name : &'static str,
	age : u8,
	pay : f32
}
fn main() {
    let mut ronny = Player{name:"ronny",age:28,pay:1_000_00.98};
    println!("ronny : {:?}",ronny );
    println!("after club change : " );
    ronny.age = 35;
    ronny.pay = 1_020_00.00;
    println!("ronny : {:?}",ronny );
    // pointers carry out automatic dereferencing .
    let r1 =  &ronny;
    println!("{:?} == {:?}",(*r1).name,r1.name);
}
