#[derive(Debug)]
struct Alien{
    name:&'static str
}
fn main() {
    let xeron = Alien{name:"xeron"};
    println!("name : {:?}",xeron );
    //let kyro = xeron;
    println!("name : {:?}",xeron );
    // ============= Copy Trait ==================
    let n:u32 = 42;
// once you have copy trait impl , you don't have to
// worry abt ownership- borrowing mechanism ,
// as the resources get a new copy on each call.
    let m = n; // new memory will be allocated .
    println!("n: {:?} ",n);
}

