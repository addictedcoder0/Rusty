fn main() {
    let aliens:[&'static str;3] = ["acsd","vrvrf","dfrg"];
    let mut rng = 0..5;
    println!("rng > {:?}",rng.next().unwrap() );
    println!("rng > {:?}",rng.next().unwrap() );
    for i in aliens.iter(){
    	println!("alien :{:?}",i );
    }
    for i in &aliens{
    	println!("alien :{:?}",i );
    }
}
