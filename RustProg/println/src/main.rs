fn main() {
    println!("Hello {}","Rudra");
    println!("you are currently in : {place}",place="chennai");
    println!("tasks:{0} and meetings:{1},completed:{1}",3,2 );
    //   formatting possibilities
    let inum:i32 = 15;
    let fnum:f64 = 15.0;
    println!("o for octal => {:o}",inum );
    println!("x for lowerHexadecimal => {:x}",inum );
    println!("x for upperHexadecimal => {:X}",inum );
    println!("p for pointer => {:p}",&inum );
    println!("b for binary => {:b}",inum );
    println!("e for lower Exponential notation {:e}",fnum );
    println!("E for upper Exponential notation {:E}",fnum );
    println!("? for Debugging {:?}",inum );
}
